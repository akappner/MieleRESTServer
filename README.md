# Miele REST Server

[![Tests](https://github.com/hannesdelbeke/MieleRESTServer/actions/workflows/rust.yml/badge.svg)](https://github.com/hannesdelbeke/MieleRESTServer/actions/workflows/rust.yml)

Miele REST Server provides local (non-cloud) control of Miele@home appliances by exposing a REST API.

## Choose your setup

1. Home Assistant add-on:
   - Use this if Home Assistant is your runtime.
   - Guide: [`docs/home-assistant.md`](docs/home-assistant.md)
2. Standalone server:
   - Use this if you want to run directly on Linux/Unix.
   - Guide: [`docs/standalone-setup.md`](docs/standalone-setup.md)

## Documentation

- Docs index: [`docs/README.md`](docs/README.md)
- API usage: [`docs/api-usage.md`](docs/api-usage.md)
- Compatibility list: [`docs/compatibility.md`](docs/compatibility.md)
- Further reading: [`docs/references.md`](docs/references.md)
- Disclaimer: [`docs/disclaimer.md`](docs/disclaimer.md)

## Quick API check

After setup, verify the server responds:

```text
GET http://{YOUR_SERVER_IP}:5001/generate-summary/
```

## Contributing

Patches and pull requests are welcome.

## License

Licensed under GPLv3. See [`LICENSE`](LICENSE).

This project is based on independent reverse engineering and is not authorized, warranted, or tested by Miele. Usage may void warranties or damage appliances. Full text: [`docs/disclaimer.md`](docs/disclaimer.md).
