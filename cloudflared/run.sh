#!/bin/sh
set -e

TOKEN=$(jq -r '.tunnel_token' /data/options.json)

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
  echo "tunnel_token is not set. Configure it in the add-on's Configuration tab." >&2
  exit 1
fi

exec cloudflared tunnel run --token "$TOKEN"
