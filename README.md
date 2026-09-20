# Webhook Proxy — Home Assistant Add-on Repository

A minimal Rust reverse proxy for Home Assistant that exposes **only** an
allow-listed set of `/api/webhook/<id>` paths to the internet. Everything
else returns 404. Built to sit behind a Cloudflare Tunnel (or any reverse
proxy) so that integrations relying on inbound webhooks — SmartThings being
the original motivating case — don't require exposing all of Home Assistant.

## Why

Home Assistant's official cloud-webhook integrations typically assume you
either pay for Nabu Casa's cloudhook, or expose your whole instance
(dashboard, API, everything) to receive the one webhook path they need. This
add-on narrows that exposure down to exactly the webhook ID(s) you configure
— nothing else is reachable through it, even with the correct hostname.

## Installation

1. In Home Assistant: **Settings → Apps → App Store** → menu (⋮) → **Repositories**
2. Add: `https://github.com/tjrmarques/ha-webhook-proxy`
3. Find **Webhook Proxy** in the store and install it
4. See [`webhook_proxy/DOCS.md`](webhook_proxy/DOCS.md) for configuration and
   Cloudflare Tunnel setup

## Repository layout

```
repository.yaml        ← tells Supervisor this is an add-on repo
webhook_proxy/          ← the add-on itself
  config.yaml           ← add-on manifest (name, options, image)
  build.yaml             ← per-arch base images (used for local builds/fallback)
  Dockerfile
  Cargo.toml
  src/main.rs
  DOCS.md
.github/workflows/build.yml  ← CI: builds + publishes multi-arch images to GHCR
```

## License

MIT — see [LICENSE](LICENSE).
