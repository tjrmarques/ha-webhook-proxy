# Webhook Proxy

Exposes only an allow-listed set of `/api/webhook/<id>` paths to whatever
sits in front of this add-on (typically a Cloudflare Tunnel). Every other
path — including all of Home Assistant's UI and API — returns 404.

## Configuration

```yaml
allowed_webhook_ids:
  - your-webhook-id-here
```

Add one entry per webhook you want reachable. Find your webhook ID under
**Settings → Devices & Services → (your integration)** — for example the
SmartThings integration's config entry shows the webhook URL it generated;
copy just the ID portion (the string after `/api/webhook/`).

With `allowed_webhook_ids` empty (the default), every request is rejected —
the add-on fails closed rather than open.

## Fronting it with a Cloudflare Tunnel

If using the Cloudflared add-on, leave its `external_hostname` field blank
(that field routes straight to Home Assistant) and use `additional_hosts`
instead, pointing at this add-on's internal hostname:

```yaml
additional_hosts:
  - hostname: ha.yourdomain.com
    service: http://webhook_proxy:8181
```

Supervisor resolves `webhook_proxy` via internal DNS to this add-on's
container, the same way `homeassistant` resolves to Home Assistant Core.

## Verifying

- `https://ha.yourdomain.com/lovelace` → 404
- `https://ha.yourdomain.com/api/webhook/<allow-listed-id>` → reaches Home Assistant
- `https://ha.yourdomain.com/api/webhook/<anything-else>` → 404
