# Miele REST Server Home Assistant Add-on

This add-on packages MieleRESTServer so Home Assistant can install and run it from the Add-on Store.

## Configuration

Preferred: configure endpoints in the add-on UI (`options.endpoints`).

Legacy fallback: if `options.endpoints` is empty, the add-on loads:

- `/config/MieleRESTServer.config`

You can use `examples/MieleRESTServer-example-config.yaml` as the template for legacy mode.

## Home Assistant YAML fragment

To create REST sensors in Home Assistant, use:

- `examples/homeassistant-configuration-fragment.yaml`
