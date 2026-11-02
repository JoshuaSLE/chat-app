# Architecture

> Status: planning. Update as decisions are made; record the reasoning in [decisions.md](decisions.md).

## Overview

Self-hosted messaging app for direct and group chat.

| Layer                   | Technology                  | Status                |
| ----------------------- | --------------------------- | --------------------- |
| Backend                 | Rust, Axum, Tokio, SQLx     | Decided               |
| Frontend                | React, TypeScript, Vite     | Decided               |
| Database                | PostgreSQL                  | Decided               |
| Object storage          | Garage (S3-compatible)      | Decided               |
| Real-time               | WebSockets                  | Planned (milestone 5) |
| Container runtime       | Podman Compose              | Planned               |
| Reverse proxy / ingress | Traefik, Cloudflare Tunnels | Planned               |
| Remote access (dev)     | Tailscale                   | Decided               |

## System diagram

_TBD. Browser -> Cloudflare Tunnel -> Traefik -> (`/` frontend static, `/api/*` backend) -> PostgreSQL / Garage._

## Single-origin routing

- `/` serves the built frontend (static assets).
- `/api/*` routes to the backend.
- Reason: same origin simplifies cookies and sessions (no CORS, `SameSite` cookies just work).
- Dev: Vite dev server proxies `/api` to the backend. _TBD: config._

## Backend

- Entry point: `backend/src/main.rs`.
- Config via environment variables (`.env` in dev via `dotenvy`; container env in production).
- Logging: `tracing` with `EnvFilter`, controlled by `RUST_LOG` (default `info`).
- Database access: SQLx connection pool; migrations embedded with `sqlx::migrate!()`.
- Module layout: _TBD (routes, handlers, services, models, errors)._
- Error handling approach: _TBD._

## Frontend

- Built to static assets, served at `/`.
- Routing, state management, API client: _TBD._

## Environments

| Environment | Database       | DB role        | Bucket | Credentials |
| ----------- | -------------- | -------------- | ------ | ----------- |
| dev         | `chat_app_dev` | `chat_app_dev` | _TBD_  | separate    |
| staging     | _TBD_          | _TBD_          | _TBD_  | separate    |
| prod        | _TBD_          | _TBD_          | _TBD_  | separate    |

- PostgreSQL and Garage instances may be shared; databases, roles, buckets, and credentials are not.
- Dev never uses production credentials or data.
- Podman Compose does not provision PostgreSQL or Garage; the app connects to existing instances.

## Deployment

_TBD (milestone 6): images, compose layout (`deploy/compose.yaml`), Traefik labels, tunnel config, CI/CD._

## Real-time design

_TBD (milestone 5): WebSocket auth, connection lifecycle, presence, typing indicators, fan-out._

## Open questions

- _Add as they come up._
