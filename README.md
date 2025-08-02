# Miele REST Server: Local Control for Miele@home Appliances

## Scope

This project implements a client for "Miele@home" based appliances, and a
REST-based server frontend to control the client. No official client exists for
the Miele@home protocol, as Miele wants you to use its "cloud".

## Setup

### 0) Resetting Miele device as needed

These instructions assume that the device is a blank configuration, and is not
currently connected to WiFi. If the device has already had step 2 or step 3
performed, it will not accept a new configuration without a reset. You can reset
the device if necessary through the local control panel.

### 1) Connecting Miele device to WiFi

Select "Miele@home" on the control panel of the Miele device. If the device offers
you a choice between "Via WPS" or "Via App", choose "Via App". 

The device will open its own access point with an SSID starting with "Miele@home".

Observe the SSID of the local access point:

#### 1a) If the SSID is "Miele@home" with no suffix, connect to the network with the PSK "secured-by-tls".

#### 1b) If the SSID is "Miele@home-{some suffix}", e.g. "Miele@home-TAA1234", connect to the network with your device's serial number shown on its sticker.

Some Miele devices run their own DHCP server. Try to receive an IP from the Miele device by running 

```
dhclient -v -i <your_wifi_interface_here>
```

If successful, observe the IP of the Miele device.

If unsuccessful, open a DHCP server to assign the Miele device an IP. You will need to run an external
DHCP server on the computer connecting to the Miele access point.

I recommend dnsmasq, e.g.:

```bash
sudo dnsmasq --port 0 --no-daemon --dhcp-range=192.168.1.100,192.168.1.200 --dhcp-leasefile=/dev/null -z --conf-file=/dev/null --interface <your_wifi_interface_here>
```

Observe the IP assigned by the DHCP server to your Miele device.

Go to the "helpers" directory.

```bash
cd helpers/
```

Edit the file "wifi.json" to contain the information of the WiFi you want the
Miele device to connect to.

*** I HIGHLY RECOMMEND FIREWALLING THIS NETWORK! ***

The Miele device will create outbound network traffic if not externally
prevented from doing so.

Run the pairing script with the IP of your Miele device. Example:

```bash
./provision-wifi.sh 10.0.0.5
```

If successful, the Miele device will close its access point and connect to the
new WiFi. Some Miele devices will use an HTTPS endpoint for provisioning. The
provisioning script attempts both, HTTP first -- one or the other will fail.

### 2) Provisioning Miele Device with Cryptographic Keys
   
   Connect to the same WiFi as the Miele device.

Generate device keys using the provided "generate-keys.py" script.

```bash
./generate-keys.py > helpers/keys.json
```

Then run the provisioning script with the IP of your Miele device. Example:

```bash
./provision-key.sh 192.168.1.50 ./keys.json
```

This will upload the generated device key to your Miele device. After this step,
your Miele device will stop responding to unencrypted/unsigned network commands.

### 3) Copy the example server configuration file in /etc/MieleRESTServer.config

Each device has a name, which is an internal handle by which the server
references it. Edit the file to enter the Miele device IPs and the same keys you
created in Step 3.

On all devices I have seen, the device route is identical to the device serial
number zero padded on the left to form a 12-digit number, e.g. "000123456789".

Specify the device route as "auto" if you do not know. If "auto", the
server will detect it upon startup, and print it in the log. You can update the
config to include the route to save the auto-detection step on startup.

### 4) Install the server 

```bash
sudo ./install.sh
```

### 5) Test the server

Test the server by navigating to: http://{YOUR_SERVER_IP}:5001/generate-summary/

You should see a JSON file that shows information from all configured devices.

### 6) Optional -- Home Assistant integration

If you want to use Home Assistant to query and display the device status, paste
the configuration fragment into your home assistant config.

## QUERYING AND SETTING DEVICE INFORMATION

After a period of inactivity, the device will go into a sleep mode. When in
sleep mode, it returns invalid data through the DOP2 endpoint. To wake the
device up from sleep, use the /wakeup/<device name> endpoint. 

Some Miele devices expose an internal binary protocol under the endpoint "DOP2".

DOP2 information can be retrieved from the /walkdop2tree/<device name>
endpoint. Not all DOP2 data structures can be decoded yet.

Individual DOP2 attributes can be read from, and written, by GET and POST
to the /dop2leaf/<device name> endpoint. 

## USING REMOTE START

Some Miele devices can be started remotely. They will only start remotely
if they are fully programmed. The server is not yet capable of programming
the device, but it can start the device if the programming is done locally.

Send a GET request to the /start/<device name> handle. This will show
whether the device reports remote start capability, and whether it is
currently programmed so that it can be remote-started.

The easiest way to program it is to set up a timer on the local control panel.

Once the timer is running, send a POST request to /start/<device name>.
Your device should start immediately, cutting the timer short.

Some devices require a configuration option ("Mode 97") set on the local control panel to allow remote start.
Counter-intuitively, this is true even if the selector knob is turned to "Remote Start". 
If remote start does not work, and the /start endpoint reports "DeviceRemoteStartCapable"
as "false", this is an indication that the additional configuration option is required.

For a video tutorial on how to set Mode 97, see:
https://www.youtube.com/watch?v=X1uq7JEM2Fg


## TODO

- Provide frontend for device configuration and program selection

Patches/pull requests welcome.

## COMPATIBILITY

Users have reported the following devices to be compatible:

Device Model | Device Type | Basic Info | Remote Start | DOP2
-------------|-------------|------------|--------------|-----
G3385	     | Dishwasher   |      X     |      ?       |  ?
G7364	     | Dishwasher   |      X     |      X       |  ?
G7366	     | Dishwasher   |      X     |      X       |  No
H7164        | Oven         |      X     |      ?       |  ?
H7240BM	     | Oven / MW    |      X     |      ?       |  ?
TWD360       | Dryer        |      X     |      ?       |  No
WTD-160	     | Washer/Dryer |      X     |      X       |  No
WWE460-WPS   | Washer       |      X     |      X       |  No
WWF360-WPS   | Washer       |      X     |      ?       |  ?
WWD660 WCS   | Washer       |      X     |      ?       |  ?
WXF660 (W1)  | Washer       |      X     |      X       |  X

Additional reports appreciated.

## FURTHER READING

API capability docs:
https://www.miele.com/developer/assets/API_V1.x.x_capabilities_by_device.pdf

Notes from Miele devs (in German):
https://community.symcon.de/t/miele-home-xkm-3100w-protokollanalyse/43633/8
(Note: The test vectors provided with the Java programs linked in this forums
are wrong. The programs themselves do not run on current versions of Java.)

https://community.home-assistant.io/t/mielerestserver-miele-home-without-cloud-possible/840093/20
Notes from users

## LICENSE AND DISCLAIMER

This program is licensed under GPLv3.

This program is solely based on independent reverse engineering and is not in any way
authorized, warranted or tested by Miele. Its use may void your warranty, or destroy
your Miele machines.

