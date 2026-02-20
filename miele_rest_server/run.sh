#!/usr/bin/env sh
set -eu

OPTIONS_FILE="/data/options.json"
LEGACY_CONFIG="/config/MieleRESTServer.config"
GENERATED_CONFIG="/tmp/MieleRESTServer.config"

if [ -f "$OPTIONS_FILE" ]; then
  python3 - <<'PY'
import json
import sys
import yaml

options_file = "/data/options.json"
generated = "/tmp/MieleRESTServer.config"

try:
    with open(options_file, "r", encoding="utf-8") as handle:
        data = json.load(handle)
except Exception as exc:
    print(f"Failed to parse {options_file}: {exc}", file=sys.stderr)
    sys.exit(1)

endpoints = data.get("endpoints") or []
if endpoints:
    cfg = {"endpoints": {}}
    for index, entry in enumerate(endpoints, start=1):
        name = str(entry.get("name", "")).strip()
        host = str(entry.get("host", "")).strip()
        group_id = str(entry.get("groupId", "")).strip()
        group_key = str(entry.get("groupKey", "")).strip()
        route = str(entry.get("route", "auto")).strip() or "auto"

        if not name:
            print(f"Endpoint #{index} is missing 'name'", file=sys.stderr)
            sys.exit(1)

        missing = [
            field_name
            for field_name, value in [
                ("host", host),
                ("groupId", group_id),
                ("groupKey", group_key),
            ]
            if not value
        ]
        if missing:
            print(
                f"Endpoint '{name}' is missing required field(s): {', '.join(missing)}",
                file=sys.stderr,
            )
            sys.exit(1)

        cfg["endpoints"][name] = {
            "host": host,
            "groupId": group_id,
            "groupKey": group_key,
            "route": route,
        }

    with open(generated, "w", encoding="utf-8") as handle:
        yaml.safe_dump(cfg, handle, sort_keys=False)
    print(f"Generated runtime config from add-on options at {generated}")
else:
    print("No endpoints configured in add-on options; falling back to legacy file mode.")
PY
fi

CONFIG_PATH="$LEGACY_CONFIG"
if [ -f "$GENERATED_CONFIG" ]; then
  CONFIG_PATH="$GENERATED_CONFIG"
  echo "Using config from add-on options: $CONFIG_PATH"
elif [ -f "$LEGACY_CONFIG" ]; then
  echo "Using legacy config file: $LEGACY_CONFIG"
else
  echo "No valid configuration found." >&2
  echo "Set add-on options.endpoints, or create $LEGACY_CONFIG" >&2
  exit 1
fi

DEBUG_ENABLED="0"
if [ -f "$OPTIONS_FILE" ]; then
  DEBUG_ENABLED="$(python3 - <<'PY'
import json

try:
    with open('/data/options.json', 'r', encoding='utf-8') as handle:
        data = json.load(handle)
except Exception:
    data = {}

print('1' if bool(data.get('debug')) else '0')
PY
)"
fi

DEBUG_ARG=""
if [ "$DEBUG_ENABLED" = "1" ]; then
  DEBUG_ARG="--debug"
fi

exec miele-rest-server -b 0.0.0.0 -p 5001 -c "$CONFIG_PATH" $DEBUG_ARG
