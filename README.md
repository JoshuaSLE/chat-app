# Chat App

A self-hosted, full-stack messaging application for direct and group communication, built with Rust, PostgreSQL, S3-compatible object storage, and React.

> **Project status:** Planning / early development. Features and interfaces described here are architectural intentions and may change.

## Highlights

- **Stack:** Rust backend (Axum, Tokio, SQLx); React, TypeScript, and Vite frontend.
- **Storage:** PostgreSQL for authoritative data; Garage (S3-compatible) for media attachments.
- **Security (planned):** Opaque server-side sessions, Argon2id password hashing, invitation-only registration.
- **Deployment (planned):** Developed locally; production services run in containers via Podman Compose, behind Traefik and Cloudflare Tunnels.
- **Environment separation:** Development, staging, and production use separate databases, database roles, storage buckets, and credentials, possibly hosted on shared PostgreSQL and Garage instances.

## Architecture Overview

- **Backend:** REST API for business logic and database access. WebSockets are planned for real-time features such as typing indicators and presence.
- **Frontend:** React app built into static assets. Traffic goes through a single origin: `/` serves the app and `/api/*` routes to the backend, simplifying session and cookie handling.
- **Infrastructure:** Podman Compose does **not** provision PostgreSQL or Garage; the application connects to existing instances. Workstations reach remote infrastructure through Tailscale. Development must never use production credentials or access production data.

## Milestones

- [ ] **1. Architecture and specification:** Finalize technical documentation and design decisions.
- [ ] **2. Infrastructure setup:** Provision per-environment PostgreSQL databases and roles, and Garage buckets and credentials.
- [ ] **3. Core services:** Database migrations, authentication, sessions, invitation-only registration.
- [ ] **4. Messaging features:** REST-based direct messaging, group conversations, membership authorization.
- [ ] **5. Real-time and media:** WebSockets, presence, typing indicators, file attachments.
- [ ] **6. Deployment and hardening:** CI/CD pipelines, security reviews, production container deployment.

## Repository Structure

```text
.
├── backend/       # Rust API, database migrations, and business logic
├── frontend/      # React, TypeScript, and Vite client application
├── deploy/        # Container configuration and deployment manifests
├── docs/          # Architecture, database, security, and API documentation
├── LICENSE
└── README.md
```

## Documentation

- [Architecture](docs/architecture.md)
- [Database](docs/database.md)
- [Security](docs/security.md)
- [API](docs/api.md)
- [Decisions](docs/decisions.md)

## Getting Started

_To be written once the first service runs. Expected prerequisites: Rust toolchain, Node.js, and access to PostgreSQL and Garage._

## License

This project is licensed under the [MIT License](LICENSE).
