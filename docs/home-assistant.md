# Home Assistant add-on setup

## Add repository and install

1. In Home Assistant, open `Settings -> Add-ons -> Add-on Store -> menu -> Repositories`.
2. Add this repository URL.
3. Install `Miele REST Server`.

Note: HACS does not install add-ons. Use the Add-on Store repository flow.

## Configure the add-on

Preferred mode: set endpoint values in add-on options (`endpoints`).

Compatibility mode: if `endpoints` is empty, create:

- `/config/MieleRESTServer.config`

Template:

- `examples/MieleRESTServer-example-config.yaml`

## Optional Home Assistant REST sensor integration

Use:

- `examples/homeassistant-configuration-fragment.yaml`

## Troubleshooting

If add-on build fails, first verify:

1. Add-on version shown in Home Assistant matches current repository version.
2. Repository URL in Home Assistant is correct.
3. Add-on rebuild is not using stale cache layers.

If you changed add-on Docker/build files, force rebuild/reinstall from the add-on page.

## References

- Add-on repository layout:
  https://developers.home-assistant.io/docs/add-ons/presentation#repository-configuration
- Add-on `config.yaml` reference:
  https://developers.home-assistant.io/docs/add-ons/configuration
- HACS docs:
  https://hacs.xyz/docs/use/
