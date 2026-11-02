# Security

> Status: planning. "Planned" items are intentions, not implemented behavior.

## Threat model

_TBD: assets (messages, attachments, credentials), actors (anonymous, authenticated user, non-member, admin), trust boundaries._

## Authentication

- **Passwords:** Argon2id hashing. Parameters: _TBD._ (Planned)
- **Registration:** invitation-only. (Planned)
  - Invite codes: random, single-use, expiring, stored hashed. Details: _TBD._
- **Login rate limiting / lockout:** _TBD._
- **Password policy and reset flow:** _TBD._

## Sessions

- Opaque server-side sessions (random token, state stored in PostgreSQL); no JWTs. (Planned)
- Token: 32 random bytes from a CSPRNG, base64url-encoded; only a hash is stored. (Planned)
- Cookie attributes: `HttpOnly`, `Secure`, `SameSite`, `Path`. Exact values: _TBD._
- Expiry (idle and absolute), rotation on login, logout/revoke-all: _TBD._
- Single-origin deployment removes the need for CORS.

## Authorization

- Every conversation/message/attachment access checks membership. (Planned)
- Role model (group admin, member): _TBD._
- Enforcement location (handler vs service layer): _TBD._

## CSRF

_TBD: depends on cookie `SameSite` choice; consider a custom header or token for state-changing requests._

## Input validation and output handling

- Validate lengths and formats at the API boundary: _TBD._
- Parameterized queries only (SQLx); no string-built SQL.
- Frontend: rely on React escaping; never render raw HTML from messages.

## WebSockets

_TBD (milestone 5): authenticate on upgrade, validate `Origin`, per-message authorization, rate limits._

## Attachments

_TBD (milestone 5): size and type limits, private bucket, presigned URLs or proxied downloads, malware considerations._

## Secrets and environments

- Dev, staging, and prod use separate databases, roles, buckets, and credentials.
- Dev never uses production credentials or data.
- `.env` files are never committed; `.env.example` lists variable names only.
- Secret storage in production: _TBD._

## Network exposure

- Public traffic enters via Cloudflare Tunnel and Traefik. (Planned)
- PostgreSQL and Garage are reachable only on the private network / Tailscale. (Planned)

## Logging and privacy

- Never log passwords, tokens, or message contents.
- Retention and deletion policy: _TBD._

## Pre-release security review checklist

- [ ] Auth flows reviewed
- [ ] Authorization checks tested for non-members
- [ ] Dependencies audited (`cargo audit`, `npm audit`)
- [ ] Secrets scan
- [ ] Headers (CSP, HSTS) configured
- [ ] Backups and restore tested
