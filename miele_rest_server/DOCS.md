# Miele REST Server Add-on

## Overview

The add-on runs MieleRESTServer and exposes the REST API on TCP port `5001`.

## Configuration

### Preferred mode (add-on options)

Set `endpoints` in the add-on configuration UI.

Each item supports:

- `name` (string, endpoint alias)
- `host` (string, appliance IP)
- `groupId` (string)
- `groupKey` (string)
- `route` (optional string, defaults to `auto`)

Optional:

- `debug` (boolean)

### Compatibility mode (legacy file)

If `endpoints` is empty, the add-on reads:

- `/config/MieleRESTServer.config`

Use `examples/MieleRESTServer-example-config.yaml` from the repository as a template.

## Logs

Startup logs state which configuration source is used:

- add-on options generated config, or
- legacy `/config/MieleRESTServer.config`.
