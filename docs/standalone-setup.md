# Standalone setup

This guide describes running Miele REST Server directly (outside Home Assistant add-on mode).

## 0) Reset the device (if needed)

These steps assume a blank device setup. If device provisioning was already done, reset the appliance on its local control panel before proceeding.

## 1) Connect the Miele device to WiFi

1. On the appliance, open `Miele@home`.
2. If prompted, choose `Via App` (not `Via WPS`).
3. Connect your computer to the temporary SSID.

SSID behavior:

- If SSID is `Miele@home` (no suffix), use PSK `secured-by-tls`.
- If SSID is `Miele@home-<suffix>`, use the appliance serial number.

Try to get an IP from the device DHCP server:

```bash
dhclient -v -i <your_wifi_interface_here>
```

If that fails, run your own DHCP server (example with `dnsmasq`):

```bash
sudo dnsmasq --port 0 --no-daemon --dhcp-range=192.168.1.100,192.168.1.200 --dhcp-leasefile=/dev/null -z --conf-file=/dev/null --interface <your_wifi_interface_here>
```

Then provision WiFi credentials:

```bash
cd helpers/
./provision-wifi.sh 192.168.0.1
```
Where `192.168.0.1` is the IP of your Miele on the temporary network.

Before provisioning, edit `helpers/wifi.json` with your target WiFi details.

Security note: firewall this network. The appliance may produce outbound traffic unless explicitly blocked.

## 2) Provision cryptographic keys

Connect to the same LAN as the appliance, then:

```bash
virtualenv venv
source venv/bin/activate
pip install -r requirements.txt
cd helpers/
../generate-keys.py > ./keys.json
./provision-key.sh 192.168.1.50 ./keys.json
```
Where `192.168.1.50` is the IP of your Miele on your LAN

After this step, the appliance no longer accepts unsigned/unencrypted commands.

## 3) Create server config

Create `/etc/MieleRESTServer.config` from `examples/MieleRESTServer-example-config.yaml`.

Each endpoint entry needs:

- `host`
- `groupId`
- `groupKey`
- `route`

`route` can be `auto` for first boot. The server prints the detected route in logs so you can persist it later.

## 4) Install server

```bash
cd ../
sudo ./install.sh
```

## 5) Verify server

Open:

```text
http://{YOUR_SERVER_IP}:5001/generate-summary/
```

You should receive JSON data for configured devices.
