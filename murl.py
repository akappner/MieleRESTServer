#!/usr/bin/python3
# Copyright (c) 2025 Alexander Kappner.
#
# This file is part of MieleRESTServer
# (see github).
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
#
import argparse
import binascii
from http import HTTPStatus
import json
import sys
from urllib.parse import urlparse

from MieleCrypto import MieleCryptoProvider, MieleProvisioningInfo
from _version import __version__

KNOWN_URLS = [
    "/",
    "/Devices",
    "/Devices/{Device-Route}",
    "/Devices/{Device-Route}/Ident",
    "/Devices/{Device-Route}/State",
    "/Devices/{Device-Route}/Settings",
    "/Diagnose",
    "/Security",
    "/Security/Commissioning",
    "/Security/HAN",
    "/Security/Cloud",
    "/Settings",
    "/Subscriptions",
    "/Update",
    "/WLAN",
]


def _format_epilog() -> str:
    return "Known Miele device URLs:\n" + "\n".join(
        f"  {url}" for url in KNOWN_URLS
    )


def _parse_url(url: str) -> tuple[str, str]:
    parsed = urlparse(url)
    scheme = parsed.scheme.lower()
    if scheme != "http":
        if scheme == "https":
            raise ValueError("https:// URLs are not supported; use http://")
        raise ValueError("URL must start with http://")
    if len(parsed.netloc) == 0:
        raise ValueError("URL must include host, e.g. http://192.168.1.50/State")

    resource_path = parsed.path.lstrip("/")
    if parsed.query:
        resource_path = (
            f"{resource_path}?{parsed.query}" if resource_path else f"?{parsed.query}"
        )

    return parsed.netloc, resource_path


def _load_provisioning_info(keys_path: str) -> MieleProvisioningInfo:
    try:
        with open(keys_path, encoding="utf-8") as handle:
            payload = handle.read()
    except OSError as exc:
        raise ValueError(f"Unable to read keys file {keys_path!r}: {exc}") from exc

    try:
        return MieleProvisioningInfo.from_paring_json(payload)
    except json.JSONDecodeError as exc:
        raise ValueError(f"Invalid JSON in keys file {keys_path!r}: {exc}") from exc
    except (KeyError, TypeError, ValueError, binascii.Error) as exc:
        raise ValueError(
            f"Invalid provisioning data in keys file {keys_path!r}: {exc}"
        ) from exc


def _decode_http_status(status_code: int) -> str:
    try:
        return HTTPStatus(status_code).phrase
    except ValueError:
        return "Unknown"


def _print_response(
    status_code: int, headers, body: bytes, include_headers: bool
) -> None:
    if include_headers:
        print(f"HTTP {status_code} {_decode_http_status(status_code)}")
        for header_name, header_value in headers.items():
            print(f"{header_name}: {header_value}")
        print()

    if len(body) == 0:
        return

    try:
        print(body.decode("utf-8"))
    except UnicodeDecodeError:
        print(f"<binary payload: {len(body)} bytes>")
        print(binascii.hexlify(body, sep=" ", bytes_per_sep=1).decode("ascii"))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="murl",
        description="Minimal encrypted HTTP client for Miele devices",
        epilog=_format_epilog(),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "-V", "--version", action="version", version="%(prog)s " + __version__
    )
    parser.add_argument(
        "-X",
        "--request",
        default="GET",
        help="HTTP method (default: GET)",
    )
    parser.add_argument(
        "-d",
        "--data",
        default=None,
        help="request body (string payload)",
    )
    parser.add_argument(
        "-i",
        "-v",
        dest="include_response_info",
        action="store_true",
        help="include response status and headers in output",
    )
    parser.add_argument(
        "-k",
        "--keys",
        default="keys.json",
        help="path to keys JSON file (default: keys.json)",
    )
    parser.add_argument("url", help="target URL (http://host/path)")
    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)

    method = args.request.strip().upper()
    if len(method) == 0:
        print("Error: HTTP method cannot be empty.", file=sys.stderr)
        return 2
    if any(ch.isspace() for ch in method):
        print(f"httpMethod must not contain whitespace: {method!r}")
        return 2

    try:
        host, resource_path = _parse_url(args.url)
        provisioning_info = _load_provisioning_info(args.keys)
    except ValueError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 2

    provider = MieleCryptoProvider(provisioning_info)
    payload = args.data if args.data is not None else ""

    try:
        decrypted, response = provider.sendHttpRequest(
            httpMethod=method,
            host=host,
            resourcePath=resource_path,
            payload=payload,
        )
    except Exception as exc:
        print(f"Request failed: {exc}", file=sys.stderr)
        return 1

    _print_response(
        response.status_code, response.headers, decrypted, args.include_response_info
    )
    if response.status_code >= 400:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
