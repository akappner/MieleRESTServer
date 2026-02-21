import unittest

import yaml

from config_wizard import (
    build_config_data,
    build_config_yaml,
    build_wifi_payload,
    normalize_endpoint_name,
    parse_iw_scan_output,
    parse_iwlist_scan_output,
    parse_nmcli_scan_output,
)


class ConfigWizardTestCase(unittest.TestCase):
    def test_normalize_endpoint_name(self):
        self.assertEqual(normalize_endpoint_name("Coffee Maker"), "coffee_maker")

    def test_build_config_yaml_matches_server_schema(self):
        config_yaml = build_config_yaml(
            endpoint_name="washer",
            host="10.0.0.11",
            group_id="1234567890ABCDEF",
            group_key="A" * 128,
            route="000123456789",
        )
        parsed = yaml.safe_load(config_yaml)
        self.assertEqual(
            parsed,
            build_config_data(
                endpoint_name="washer",
                host="10.0.0.11",
                group_id="1234567890ABCDEF",
                group_key="A" * 128,
                route="000123456789",
            ),
        )

    def test_parse_nmcli_scan_output_with_signal(self):
        output = "Home\\:Net:70:WPA2\nOffice:40:WPA3\nOffice:10:\n"
        networks = parse_nmcli_scan_output(output)
        self.assertEqual(
            networks[0], {"SSID": "Home:Net", "RSSI": 70, "Sec": "WPA2"}
        )
        self.assertEqual(
            networks[1], {"SSID": "Office", "RSSI": 40, "Sec": "WPA3"}
        )

    def test_parse_iw_scan_output(self):
        output = (
            "BSS aa:bb:cc:dd:ee:ff(on wlan0)\n"
            "\tsignal: -45.00 dBm\n"
            "\tSSID: StrongNet\n"
            "\tRSN: * Version: 1\n"
            "BSS 11:22:33:44:55:66(on wlan0)\n"
            "\tsignal: -70.00 dBm\n"
            "\tSSID: Guest\n"
        )
        networks = parse_iw_scan_output(output)
        self.assertEqual(networks[0]["SSID"], "StrongNet")
        self.assertEqual(networks[0]["Sec"], "WPA2")
        self.assertEqual(networks[1]["SSID"], "Guest")

    def test_parse_iwlist_scan_output(self):
        output = (
            'Cell 01 - Address: AA\nESSID:"Kitchen"\nSignal level=-55 dBm\n'
            "Encryption key:on\nIE: WPA Version 1\n"
            'Cell 02 - Address: BB\nESSID:"Guest"\nQuality=30/70\n'
            "Encryption key:off\n"
        )
        networks = parse_iwlist_scan_output(output)
        self.assertEqual(
            networks[0], {"SSID": "Kitchen", "RSSI": -55, "Sec": "WPA"}
        )
        self.assertEqual(networks[1]["SSID"], "Guest")
        self.assertEqual(networks[1]["Sec"], "OPEN")

    def test_build_wifi_payload_open_network(self):
        payload = build_wifi_payload(ssid="Guest", security="OPEN", wifi_key="ignored")
        self.assertEqual(payload["Key"], "")


if __name__ == "__main__":
    unittest.main()
