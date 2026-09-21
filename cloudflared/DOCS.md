# Cloudflared Tunnel

Runs `cloudflared` connected to a Cloudflare Tunnel using a **tunnel token** —
no interactive login, no `cert.pem`, no broad Cloudflare account access. The
token is scoped to exactly one tunnel.

## 1. Prerequisites (one-time, on your Cloudflare account)

- A domain added to your Cloudflare account, with Cloudflare as its DNS
  provider.
- If this is your first time touching Tunnels/Zero Trust on this account,
  Cloudflare may prompt you to select a plan before it lets you continue.
  Choose the **Free** tier (up to 50 users) — there's no charge. Some
  accounts are also asked for a payment method on file as a verification
  step even on the free tier; this is not a charge, just card-on-file.

## 2. Create the tunnel

1. In the Cloudflare dashboard, go to **Networking → Tunnels** (on some
   accounts this instead shows as **Zero Trust → Networks → Tunnels** —
   same feature, different menu path depending on account/dashboard
   version).
2. **Create a tunnel** → choose **Cloudflared** as the connector.
3. Name it (e.g. `homeassistant`).
4. Cloudflare shows you an install command containing a long token
   (`cloudflared tunnel run --token eyJ...`) — copy just the token string.
5. Under **Public Hostname**, add a route:
   - **Subdomain**: `ha`
   - **Domain**: pick your domain from the dropdown
   - **Service**: `HTTP` → `webhook_proxy:8181`

   You do **not** need to create the `ha.yourdomain.com` DNS record
   yourself first — adding this Public Hostname creates the matching DNS
   record (a proxied `CNAME` pointing at your tunnel) automatically.

   This is also where all routing lives going forward — nothing in this
   add-on needs to know about hostnames or ingress rules, that's all
   configured here, remotely.

## 3. Configure the add-on

In this add-on's **Configuration** tab:

```yaml
tunnel_token: "eyJ...your token..."
```

Start the add-on. Check the **Log** tab — `cloudflared` should report the
tunnel as connected. You can also confirm this from the Cloudflare
dashboard: the tunnel's status should show as **Healthy**/connected.

## 4. Verify

- `https://ha.yourdomain.com/lovelace` → 404 (handled by the Webhook Proxy
  add-on this tunnel routes to)
- `https://ha.yourdomain.com/api/webhook/<allow-listed-id>` → reaches Home
  Assistant

## Changing routes later

Add, remove, or repoint hostnames anytime from the same **Public Hostname**
screen in the Cloudflare dashboard — no add-on restart or reconfiguration
needed here.
