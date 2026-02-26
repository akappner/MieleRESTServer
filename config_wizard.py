#!/usr/bin/python3
# Copyright (c) 2026.
#
# This file is part of MieleRESTServer.

import argparse
import json
import os
import re
import secrets
import socket
import subprocess
from datetime import datetime
from ipaddress import IPv4Address, ip_address
from pathlib import Path

import requests
import yaml
from flask import Flask, Response, redirect, render_template, request, session, url_for
from urllib3.exceptions import InsecureRequestWarning

from MieleCrypto import MieleProvisioningInfo, MieleCryptoProvider

requests.packages.urllib3.disable_warnings(category=InsecureRequestWarning)

APPLIANCE_TYPES = [
    "washer",
    "dryer",
    "dishwasher",
    "oven",
    "coffee",
    "washer_dryer",
    "fridge",
    "freezer",
]

OPEN_SECURITY_VALUES = {"", "open", "none", "--"}

URI_AVAILABLE_STATUSES = {200, 204, 301, 302, 401, 403, 405, 501}

KEYS_FILE = "keys.json"


def resolve_template_dir():
    for candidate in (
        Path.cwd() / "templates",
        Path(__file__).resolve().parent / "templates",
    ):
        if candidate.is_dir():
            return str(candidate)
    return "templates"


def _run_command(args, timeout=12):
    try:
        completed = subprocess.run(
            args,
            check=False,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return completed.returncode, completed.stdout, completed.stderr
    except FileNotFoundError as exc:
        return 127, "", str(exc)
    except subprocess.TimeoutExpired:
        return 124, "", f"Command timed out: {' '.join(args)}"


def _unescape_nmcli(value):
    return value.replace(r"\:", ":").replace(r"\\", "\\")


def _as_int(value):
    try:
        return int(round(float(value)))
    except (TypeError, ValueError):
        return None


def _sort_networks_by_signal(networks):
    return sorted(
        networks,
        key=lambda item: (
            item.get("RSSI") is None,
            -(item.get("RSSI") if item.get("RSSI") is not None else -9999),
            item["SSID"].lower(),
        ),
    )


def _merge_network_record(existing, candidate):
    if existing is None:
        return candidate

    existing_signal = existing.get("RSSI")
    candidate_signal = candidate.get("RSSI")

    if existing_signal is None and candidate_signal is not None:
        return candidate
    if (
        existing_signal is not None
        and candidate_signal is not None
        and candidate_signal > existing_signal
    ):
        return candidate

    if existing.get("Sec", "") in {"", "UNKNOWN"} and candidate.get("Sec", "") not in {
        "",
        "UNKNOWN",
    }:
        return candidate

    return existing


def parse_nmcli_scan_output(output):
    networks = {}
    for line in output.splitlines():
        line = line.strip()
        if not line:
            continue

        parts = re.split(r"(?<!\\):", line, maxsplit=2)
        if len(parts) == 3:
            ssid_raw, signal_raw, security_raw = parts
        elif len(parts) == 2:
            ssid_raw, security_raw = parts
            signal_raw = ""
        else:
            ssid_raw = parts[0]
            signal_raw = ""
            security_raw = ""

        ssid = _unescape_nmcli(ssid_raw).strip()
        if not ssid:
            continue

        signal = _as_int(signal_raw)
        security = security_raw.strip() or "UNKNOWN"

        candidate = {"SSID": ssid, "RSSI": signal, "Sec": security}
        networks[ssid] = _merge_network_record(networks.get(ssid), candidate)

    return _sort_networks_by_signal(list(networks.values()))


def _infer_iw_security(block):
    if re.search(r"RSN", block):
        if re.search(r"SAE", block, flags=re.IGNORECASE):
            return "WPA3/WPA2"
        return "WPA2"
    if re.search(r"\bWPA\b", block):
        return "WPA"
    if re.search(r"capability:.*privacy", block, flags=re.IGNORECASE):
        return "WEP"
    return "OPEN"


def parse_iw_scan_output(output):
    networks = {}
    for block in re.split(r"(?=^BSS\s)", output, flags=re.MULTILINE):
        ssid_match = re.search(r"^\s*SSID:\s*(.+)$", block, flags=re.MULTILINE)
        if not ssid_match:
            continue
        ssid = ssid_match.group(1).strip()
        if not ssid:
            continue

        signal_match = re.search(r"signal:\s*(-?\d+(?:\.\d+)?)\s*dBm", block)
        signal = _as_int(signal_match.group(1)) if signal_match else None
        security = _infer_iw_security(block)

        candidate = {"SSID": ssid, "RSSI": signal, "Sec": security}
        networks[ssid] = _merge_network_record(networks.get(ssid), candidate)

    return _sort_networks_by_signal(list(networks.values()))


def parse_iwlist_scan_output(output):
    networks = {}
    for block in output.split("Cell "):
        essid_match = re.search(r'ESSID:"([^"]+)"', block)
        if not essid_match:
            continue

        ssid = essid_match.group(1).strip()
        if not ssid:
            continue

        signal = None
        level_match = re.search(r"Signal level=(-?\d+)\s*dBm", block)
        if level_match:
            signal = _as_int(level_match.group(1))
        else:
            quality_match = re.search(r"Quality=(\d+)/(\d+)", block)
            if quality_match:
                numerator = _as_int(quality_match.group(1))
                denominator = _as_int(quality_match.group(2))
                if numerator is not None and denominator:
                    # Convert quality ratio to a dBm-like value for sortable ranking.
                    ratio = numerator / denominator
                    signal = int(round(-100 + ratio * 60))

        security = "OPEN" if "Encryption key:off" in block else "UNKNOWN"
        if re.search(r"WPA2|WPA Version 2", block, flags=re.IGNORECASE):
            security = "WPA2"
        elif re.search(r"\bWPA\b", block, flags=re.IGNORECASE):
            security = "WPA"

        candidate = {"SSID": ssid, "RSSI": signal, "Sec": security}
        networks[ssid] = _merge_network_record(networks.get(ssid), candidate)

    return _sort_networks_by_signal(list(networks.values()))


def _discover_wireless_interfaces():
    interfaces = []

    rc, stdout, _ = _run_command(["iw", "dev"])
    if rc == 0:
        for line in stdout.splitlines():
            match = re.search(r"^\s*Interface\s+(\S+)", line)
            if match:
                interfaces.append(match.group(1))

    if interfaces:
        return sorted(set(interfaces))

    net_dir = Path("/sys/class/net")
    if net_dir.exists():
        for entry in net_dir.iterdir():
            if entry.name.startswith(("wl", "wlan")):
                interfaces.append(entry.name)

    return sorted(set(interfaces))


def _scan_with_iw():
    errors = []
    merged = {}

    interfaces = _discover_wireless_interfaces()
    if not interfaces:
        return [], ["Could not detect wireless interfaces for iw scan."]

    for iface in interfaces:
        rc, stdout, stderr = _run_command(["iw", "dev", iface, "scan"], timeout=18)
        if rc != 0:
            msg = stderr.strip() or "unknown error"
            errors.append(f"iw scan failed on {iface}: {msg}")
            continue

        for network in parse_iw_scan_output(stdout):
            ssid = network["SSID"]
            merged[ssid] = _merge_network_record(merged.get(ssid), network)

    if merged:
        return _sort_networks_by_signal(list(merged.values())), errors

    return [], errors or ["iw scan returned no SSIDs."]


def _scan_with_iwlist():
    errors = []
    merged = {}

    interfaces = _discover_wireless_interfaces()
    if not interfaces:
        return [], ["Could not detect wireless interfaces for iwlist scan."]

    for iface in interfaces:
        rc, stdout, stderr = _run_command(["iwlist", iface, "scanning"], timeout=18)
        if rc != 0:
            msg = stderr.strip() or "unknown error"
            errors.append(f"iwlist scan failed on {iface}: {msg}")
            continue

        for network in parse_iwlist_scan_output(stdout):
            ssid = network["SSID"]
            merged[ssid] = _merge_network_record(merged.get(ssid), network)

    if merged:
        return _sort_networks_by_signal(list(merged.values())), errors

    return [], errors or ["iwlist scan returned no SSIDs."]


def scan_networks(prefix=None):
    errors = []

    rc, stdout, stderr = _run_command(
        ["nmcli", "-t", "-f", "SSID,SIGNAL,SECURITY", "dev", "wifi", "list"]
    )
    if rc == 0:
        networks = parse_nmcli_scan_output(stdout)
        if networks:
            return _filter_networks(networks, prefix), "nmcli", errors
        errors.append("nmcli returned no SSIDs.")
    else:
        msg = stderr.strip() or "unknown error"
        errors.append(f"nmcli scan failed: {msg}")

    networks, iw_errors = _scan_with_iw()
    errors.extend(iw_errors)
    if networks:
        return _filter_networks(networks, prefix), "iw", errors

    networks, iwlist_errors = _scan_with_iwlist()
    errors.extend(iwlist_errors)
    if networks:
        return _filter_networks(networks, prefix), "iwlist", errors

    return [], "manual", errors


def _filter_networks(networks, prefix):
    if not prefix:
        return networks
    return [
        item for item in networks if item["SSID"].lower().startswith(prefix.lower())
    ]


def get_network_snapshot():
    snapshot = {}
    errors = []

    rc, stdout, stderr = _run_command(["ip", "-j", "-4", "addr", "show"])
    if rc != 0:
        msg = stderr.strip() or "unknown error"
        errors.append(f"Could not inspect interfaces: {msg}")
        return snapshot, errors

    try:
        interfaces = json.loads(stdout)
    except json.JSONDecodeError:
        errors.append("Could not parse interface information from ip command output.")
        return snapshot, errors

    for entry in interfaces:
        iface = entry.get("ifname")
        if not iface:
            continue
        addresses = [
            addr.get("local")
            for addr in entry.get("addr_info", [])
            if addr.get("family") == "inet" and addr.get("local")
        ]
        snapshot[iface] = {"addresses": sorted(addresses), "gateway": ""}

    rc, stdout, stderr = _run_command(["ip", "-j", "route", "show", "default"])
    if rc != 0:
        msg = stderr.strip() or "unknown error"
        errors.append(f"Could not inspect default route: {msg}")
        return snapshot, errors

    try:
        routes = json.loads(stdout)
    except json.JSONDecodeError:
        errors.append("Could not parse default route information.")
        return snapshot, errors

    for route in routes:
        iface = route.get("dev")
        gateway = route.get("gateway", "")
        if not iface:
            continue
        snapshot.setdefault(iface, {"addresses": [], "gateway": ""})
        if gateway:
            snapshot[iface]["gateway"] = gateway

    return snapshot, errors


def _is_wireless_interface_name(name):
    return name.startswith(("wl", "wlan"))


def detect_new_connection(before_snapshot, after_snapshot):
    candidates = []

    for iface, current in after_snapshot.items():
        addresses = tuple(current.get("addresses", []))
        if not addresses:
            continue

        previous = before_snapshot.get(iface, {})
        previous_addresses = tuple(previous.get("addresses", []))

        if addresses != previous_addresses:
            score = 2
        elif current.get("gateway") and current.get("addresses"):
            score = 1
        else:
            score = 0

        if _is_wireless_interface_name(iface):
            score += 1

        candidates.append((score, iface, current))

    if not candidates:
        return "", {"addresses": [], "gateway": ""}

    candidates.sort(
        key=lambda item: (item[0], len(item[2].get("addresses", []))), reverse=True
    )
    _, iface, details = candidates[0]
    return iface, details


def normalize_endpoint_name(appliance_type):
    normalized = re.sub(r"[^a-z0-9_-]+", "_", (appliance_type or "").lower())
    normalized = normalized.strip("_")
    if not normalized:
        raise ValueError("Appliance type must contain letters or digits.")
    return normalized


def validate_ipv4(host):
    value = (host or "").strip()
    parsed = ip_address(value)
    if not isinstance(parsed, IPv4Address):
        raise ValueError("Only IPv4 addresses are currently supported.")
    return str(parsed)


def is_open_security(security):
    normalized = (security or "").strip().lower()
    if normalized in OPEN_SECURITY_VALUES:
        return True
    return "open" in normalized and "wpa" not in normalized


def normalize_security(security):
    value = (security or "").strip()
    if not value or value.upper() == "UNKNOWN":
        return "WPA2"
    return value


def build_wifi_payload(ssid, security, wifi_key):
    sec_value = normalize_security(security)
    return {
        "SSID": ssid,
        "Sec": sec_value,
        "Key": "" if is_open_security(sec_value) else wifi_key,
    }


def build_config_data(endpoint_name, host, group_id, group_key, route):
    return {
        "endpoints": {
            endpoint_name: {
                "host": host,
                "groupId": group_id,
                "groupKey": group_key,
                "route": route,
            }
        }
    }


def build_config_yaml(endpoint_name, host, group_id, group_key, route):
    config = build_config_data(endpoint_name, host, group_id, group_key, route)
    return yaml.safe_dump(config, sort_keys=False, indent=4)


def _tcp_port_open(host, port=80, timeout=3):
    try:
        with socket.create_connection((host, port), timeout=timeout):
            return True, f"TCP {host}:{port} is reachable."
    except OSError as exc:
        return False, f"TCP {host}:{port} check failed: {exc}"


def _check_uri_available(host, path, use_https=False, headers=None):
    scheme = "https" if use_https else "http"
    url = f"{scheme}://{host}{path}"

    try:
        response = requests.get(
            url,
            headers=headers or {},
            timeout=5,
            allow_redirects=False,
            verify=not use_https,
        )
    except requests.RequestException as exc:
        return False, f"GET {url} failed: {exc}"

    status = response.status_code
    if status in URI_AVAILABLE_STATUSES:
        return True, f"GET {url} -> HTTP {status}"

    return False, f"GET {url} -> HTTP {status}"


def check_wlan_uri(host):
    return _check_uri_available(host, "/WLAN", use_https=False)


def check_commissioning_uri(host):
    checks = []

    ok, message = _check_uri_available(host, "/Security/Commissioning", use_https=False)
    checks.append(message)
    if ok:
        return True, checks

    ok, message = _check_uri_available(
        host,
        "/Security/Commissioning",
        use_https=True,
        headers={"Authorization": "MielePairing:Pairing"},
    )
    checks.append(message)
    return ok, checks


def get_miele_visible_networks(host):
    url = f"http://{host}/WLAN/Scan"

    try:
        response = requests.get(url)
    except requests.RequestException as exc:
        return False, f"Get {url} failed: {exc}"

    data = response.json()
    return _sort_networks_by_signal(data["Result"])


def provision_wifi(host, wifi_payload):
    url = f"http://{host}/WLAN"

    try:
        response = requests.put(
            url,
            data=json.dumps(wifi_payload),
            headers={"Content-Type": "application/json"},
            timeout=8,
        )
    except requests.RequestException as exc:
        return False, f"PUT {url} failed: {exc}"

    status = response.status_code
    snippet = response.text.strip().replace("\n", " ")[:220]
    if 200 <= status < 300:
        msg = f"PUT {url} -> HTTP {status}"
        if snippet:
            msg += f" ({snippet})"
        return True, msg

    msg = f"PUT {url} -> HTTP {status}"
    if snippet:
        msg += f" ({snippet})"
    return False, msg


def generate_keys_payload():
    mpi = MieleProvisioningInfo.generate_random()
    mpi_dict = mpi.to_dict()

    group_id = mpi_dict["GroupID"]
    group_key = mpi_dict["GroupKey"]
    payload = mpi.to_pairing_json()

    return group_id, group_key, payload


def provision_keys(host, keys_payload):
    attempts = []

    base_url = f"{host}/Security/Commissioning"
    http_url = f"http://{base_url}"
    https_url = f"https://{base_url}"

    try:
        response = requests.put(http_url, data=keys_payload, timeout=8)
        attempts.append(f"PUT {http_url} -> HTTP {response.status_code}")
        if 200 <= response.status_code < 300:
            return True, attempts
    except requests.RequestException as exc:
        attempts.append(f"PUT {http_url} failed: {exc}")

    try:
        response = requests.put(
            https_url,
            data=keys_payload,
            headers={"Authorization": "MielePairing:Pairing"},
            timeout=8,
            verify=False,
        )
        attempts.append(f"PUT {https_url} -> HTTP {response.status_code}")
        if 200 <= response.status_code < 300:
            return True, attempts
    except requests.RequestException as exc:
        attempts.append(f"PUT {https_url} failed: {exc}")

    return False, attempts


def fetch_device_route(group_id: str, group_key: str, device_ip: str):
    provisioning_info = MieleProvisioningInfo(group_id, group_key)
    crypto_provider = MieleCryptoProvider(provisioning_info)

    response = crypto_provider.sendHttpRequest(host=device_ip, resourcePath="Devices")[
        0
    ]

    j = json.loads(response)
    if len(j.keys()) == 1:
        device_route = list(j.keys())[0]
        return device_route
    return None


def _get_state():
    return dict(session.get("wizard", {}))


def _save_state(state):
    session["wizard"] = state
    session.modified = True


def _require_state_fields(*fields):
    state = _get_state()
    return all(state.get(field) for field in fields)


def create_app():
    app = Flask(__name__, template_folder=resolve_template_dir())
    app.secret_key = os.environ.get("MIELE_WIZARD_SECRET") or secrets.token_hex(32)

    @app.route("/", methods=["GET"])
    def index():
        return redirect(url_for("precond"))

    @app.route("/precond", methods=["GET", "POST"])
    def precond():
        errors = []

        if request.method == "POST":
            action = request.form.get("action", "")
            if action == "initial-setup":
                if request.form.get("reset_confirmed") != "yes":
                    errors.append(
                        "Confirm that the appliance has been reset before continuing."
                    )
                else:
                    state = {"reset_confirmed": True}
                    _save_state(state)
                    return redirect(url_for("miele_ap"))
            elif action == "commissioning":
                if request.form.get("commissioning") != "yes":
                    errors.append(
                        "Confirm that the appliance has been connected to WiFi before continuing."
                    )
                else:
                    state = {"wifi_provisioned": True}
                    _save_state(state)
                    return redirect(url_for("commissioning"))
            elif action == "config_preview":
                if request.form.get("config_preview") != "yes":
                    errors.append(
                        "Confirm that the appliance has been previously commissioned "
                        "using this wizard before continuing."
                    )
                else:
                    state = {"key_provisioned": True}
                    _save_state(state)
                    return redirect(url_for("config_preview"))

        return render_template(
            "config_wizard/precond.html",
            title="Step 1: Preconditions",
            errors=errors,
        )

    @app.route("/miele-ap", methods=["GET", "POST"])
    def miele_ap():
        if not _require_state_fields("reset_confirmed"):
            return redirect(url_for("precond"))

        state = _get_state()
        errors = []
        diagnostics = []

        if "pre_ap_snapshot" not in state:
            snapshot, snapshot_errors = get_network_snapshot()
            state["pre_ap_snapshot"] = snapshot
            _save_state(state)
            diagnostics.extend(snapshot_errors)

        if request.method == "POST":

            if request.form.get("ap_connected_confirmed") != "yes":
                errors.append("Confirm that this host is connected to the Miele AP.")

            current_snapshot, snapshot_errors = get_network_snapshot()
            diagnostics.extend(snapshot_errors)

            inferred_iface, inferred_details = detect_new_connection(
                state.get("pre_ap_snapshot", {}), current_snapshot
            )
            inferred_gateway = inferred_details.get("gateway", "")

            candidate_host = inferred_gateway

            if not candidate_host:
                errors.append("Could not infer Miele device address from gateway.")

            if candidate_host:
                try:
                    candidate_host = validate_ipv4(candidate_host)
                except ValueError as exc:
                    errors.append(str(exc))

            if candidate_host and not errors:
                tcp_ok, tcp_message = _tcp_port_open(candidate_host, 80)
                diagnostics.append(tcp_message)
                if not tcp_ok:
                    errors.append(
                        f"Could not reach TCP port 80 on inferred host {candidate_host}."
                    )

            if candidate_host and not errors:
                wlan_ok, wlan_message = check_wlan_uri(candidate_host)
                diagnostics.append(wlan_message)
                if not wlan_ok:
                    errors.append(
                        "GET /WLAN did not look available on the inferred host."
                    )

            if not errors:
                state["miele_ap_interface"] = inferred_iface
                state["miele_ap_gateway"] = inferred_gateway
                state["miele_ap_host"] = candidate_host
                state["miele_ap_verified"] = True
                _save_state(state)
                return redirect(url_for("wifi_target"))

            _save_state(state)

        networks, scan_source, scan_errors = scan_networks(prefix="Miele@home")
        diagnostics.extend(scan_errors)

        return render_template(
            "config_wizard/miele_ap.html",
            title="Step 2: Connect To Miele AP",
            errors=errors,
            diagnostics=diagnostics,
            state=state,
            networks=networks,
            scan_source=scan_source,
        )

    @app.route("/wifi-target", methods=["GET", "POST"])
    def wifi_target():
        if not _require_state_fields("miele_ap_verified", "miele_ap_host"):
            return redirect(url_for("miele_ap"))

        state = _get_state()
        errors = []
        diagnostics = []

        if request.method == "POST":
            action = request.form.get("action", "")

            if action == "select":
                selected_ssid = request.form.get("selected_ssid", "").strip()
                selected_security = request.form.get("selected_security", "").strip()
                if selected_ssid:
                    state["target_ssid"] = selected_ssid
                    state["target_security"] = normalize_security(selected_security)
                    _save_state(state)

            elif action == "provision":
                ssid = request.form.get("target_ssid", "").strip()
                security = normalize_security(request.form.get("target_security", ""))
                wifi_key = request.form.get("target_wifi_key", "")

                if not ssid:
                    errors.append("Target Wi-Fi SSID is required.")
                if not is_open_security(security) and not wifi_key:
                    errors.append("Wi-Fi password is required for secured networks.")

                if not errors:
                    payload = build_wifi_payload(ssid, security, wifi_key)
                    success, message = provision_wifi(state["miele_ap_host"], payload)
                    diagnostics.append(message)

                    if success:
                        state["target_ssid"] = ssid
                        state["target_security"] = security
                        state["target_wifi_key"] = wifi_key
                        state["wifi_provisioned"] = True
                        state["wifi_provision_message"] = message
                        _save_state(state)
                        return redirect(url_for("commissioning"))

                    errors.append("Wi-Fi provisioning request failed.")

        networks = get_miele_visible_networks(state["miele_ap_host"])
        if not networks:
            networks, _, scan_errors = scan_networks()
            diagnostics.extend(scan_errors)

        return render_template(
            "config_wizard/wifi_target.html",
            title="Step 3: Setup Wi-Fi Network",
            errors=errors,
            diagnostics=diagnostics,
            state=state,
            networks=networks,
        )

    @app.route("/commissioning", methods=["GET", "POST"])
    def commissioning():
        if not _require_state_fields("wifi_provisioned"):
            return redirect(url_for("wifi_target"))

        state = _get_state()
        errors = []
        diagnostics = []

        if request.method == "POST":
            connected_main_wifi = request.form.get("connected_main_wifi") == "yes"
            device_ip_raw = request.form.get("device_ip", "").strip()

            if not connected_main_wifi:
                errors.append("Confirm that this host is connected to the main Wi-Fi.")

            try:
                device_ip = validate_ipv4(device_ip_raw)
            except ValueError as exc:
                errors.append(str(exc))

            if not errors:
                available, checks = check_commissioning_uri(device_ip)
                diagnostics.extend(checks)
                if not available:
                    errors.append(
                        "Could not verify /Security/Commissioning availability on the "
                        "provided IP."
                    )

            if not errors:
                try:
                    group_id, group_key, keys_payload = generate_keys_payload()
                    keys_file = KEYS_FILE
                    if Path(keys_file).exists():
                        now = datetime.now()
                        keys_file = f"keys-{now.strftime("%Y%m%dT%H%M%S")}.json"
                    with open(keys_file, "wt") as fp:
                        fp.write(keys_payload)
                    print(f"Saved keys into {keys_file} in {Path.cwd()}")

                except ValueError as exc:
                    errors.append(str(exc))
                    group_id = ""
                    group_key = ""
                    keys_payload = ""

            if not errors:
                success, attempts = provision_keys(device_ip, keys_payload)
                diagnostics.extend(attempts)
                if not success:
                    errors.append("Key provisioning failed over both HTTP and HTTPS.")

            if not errors:
                state["group_id"] = group_id
                state["group_key"] = group_key
                state["keys_json"] = keys_payload
                state["device_ip"] = device_ip
                state["key_provisioned"] = True
                state["commissioning_diagnostics"] = diagnostics
                _save_state(state)
                return redirect(url_for("config_preview"))

        return render_template(
            "config_wizard/commissioning.html",
            title="Step 4: Provision Keys",
            errors=errors,
            diagnostics=diagnostics,
            state=state,
        )

    @app.route("/config", methods=["GET", "POST"])
    def config_preview():
        required = (
            "key_provisioned",
            "device_ip",
            "group_id",
            "group_key",
        )
        keys_required = ("group_id", "group_key")
        settings_required = ("appliance_type", "device_route")

        errors = []
        state = _get_state()

        if not _require_state_fields(*keys_required) and _require_state_fields(
            "key_provisioned"
        ):
            # shortcut, attempt to load keys from KEYS_FILE
            try:
                with open(KEYS_FILE, "rt") as fp:
                    payload = fp.read()
                mpi = MieleProvisioningInfo.from_paring_json(payload)
                mpi_dict = mpi.to_dict()
                state["group_id"] = mpi_dict["GroupID"]
                state["group_key"] = mpi_dict["GroupKey"]
                state["keys_json"] = payload
                _save_state(state)
            except Exception as exc:
                errors.append(str(exc))
        elif not _require_state_fields(*required) and request.method == "GET":
            return redirect(url_for("commissioning"))

        if request.method == "POST":
            action = request.form.get("action", "")
            if action == "cleanup":
                _save_state({})
            else:
                endpoint_name = ""
                device_ip = ""
                route_value = ""

                appliance_type_choice = request.form.get("appliance_type", "").strip()
                appliance_type_custom = request.form.get(
                    "appliance_type_custom", ""
                ).strip()
                route_override = request.form.get("route_override", "").strip()
                device_ip_raw = request.form.get("device_ip", "").strip()

                appliance_type_value = (
                    appliance_type_custom
                    if appliance_type_choice == "__custom__"
                    else appliance_type_choice
                )

                try:
                    endpoint_name = normalize_endpoint_name(appliance_type_value)
                except ValueError as exc:
                    errors.append(str(exc))

                if device_ip_raw:
                    try:
                        device_ip = validate_ipv4(device_ip_raw)
                    except ValueError as exc:
                        errors.append(str(exc))

                if route_override:
                    if route_override.lower() == "auto":
                        route_value = "auto"
                    elif re.fullmatch(r"\d{12}", route_override):
                        route_value = route_override
                    else:
                        errors.append('Route override must be "auto" or 12 digits.')

                if not errors:
                    state["appliance_type"] = endpoint_name
                    state["route_override"] = route_value
                    if device_ip_raw:
                        state["device_ip"] = device_ip
                    _save_state(state)
                    return redirect(url_for("config_preview"))

        if _require_state_fields(*keys_required, "device_ip"):
            try:
                device_route = fetch_device_route(
                    state["group_id"], state["group_key"], state["device_ip"]
                )
                if device_route:
                    state["derived_device_route"] = device_route
            except Exception as exc:
                errors.append(f"Getting device route failed: {exc}")

        device_route = state.get("route_override")
        if not device_route:
            device_route = state.get("derived_device_route", "auto")
        state["device_route"] = device_route
        _save_state(state)

        config_yaml = None
        if _require_state_fields(*keys_required, *settings_required, "device_ip"):
            config_yaml = build_config_yaml(
                endpoint_name=state["appliance_type"],
                host=state["device_ip"],
                group_id=state["group_id"],
                group_key=state["group_key"],
                route=state["device_route"],
            )

        keys_json = state.get("keys_json")

        return render_template(
            "config_wizard/config_preview.html",
            title="Step 5: Save Config",
            state=state,
            errors=errors,
            appliance_types=APPLIANCE_TYPES,
            config_yaml=config_yaml,
            keys_json=keys_json,
            keys_file=KEYS_FILE,
            cur_dir=Path.cwd(),
        )

    return app


def main(argv: list[str] | None = None):
    parser = argparse.ArgumentParser(description="MieleRESTServer configuration wizard")
    parser.add_argument("-b", "--bind", default="127.0.0.1", help="bind address")
    parser.add_argument("-p", "--port", type=int, default=5081, help="bind port")
    parser.add_argument("--debug", action="store_true", help="enable Flask debug mode")

    args = parser.parse_args(argv)
    app = create_app()
    app.run(host=args.bind, port=args.port, debug=args.debug)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
