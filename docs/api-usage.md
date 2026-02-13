# API usage

## Device querying and control

After inactivity, devices can enter sleep mode and return invalid DOP2 data. Use:

```text
/wakeup/<device_name>
```

before DOP2 reads when needed.

Some Miele devices expose a binary protocol endpoint called `DOP2`.

Useful endpoints:

- DOP2 tree walk:
  - `GET /walkdop2tree/<device_name>`
- Read/write DOP2 leaf:
  - `GET /dop2leaf/<device_name>/<unit>/<attribute>`
  - `POST /dop2leaf/<device_name>/<unit>/<attribute>`
- Summary:
  - `GET /generate-summary`
  - `GET /generate-summary/<device_name>`

## Remote start

Remote start only works if the appliance is fully programmed locally.

Endpoints:

- Capability/status:
  - `GET /start/<device_name>`
- Trigger start:
  - `POST /start/<device_name>`

Common workflow:

1. Program a timer on the appliance.
2. Call `POST /start/<device_name>`.

Some devices additionally require local option `Mode 97`, even when the selector is on `Remote Start`.

If `GET /start/<device_name>` reports `DeviceRemoteStartCapable` as `false`, verify this local setting.

Video walkthrough for Mode 97:

- https://www.youtube.com/watch?v=X1uq7JEM2Fg
