# Decisions

Log of significant technical decisions. Add a new entry at the top of the log for each one; don't rewrite old entries. If a decision changes, add a new entry that supersedes the old one and update the old entry's status.

**Statuses:** Accepted, Proposed, Superseded, Rejected.

## Template

### NNN. Title

- **Date:** YYYY-MM-DD
- **Status:** Proposed | Accepted | Superseded by NNN
- **Context:** What problem or constraint prompted this?
- **Decision:** What was chosen?
- **Alternatives considered:** Options and why they were not chosen.
- **Consequences:** Trade-offs, follow-up work, what gets harder or easier.

## Index

| #   | Decision                               | Status   |
| --- | -------------------------------------- | -------- |
| 001 | Backend and frontend stack             | Accepted |
| 002 | Single-origin routing                  | Accepted |
| 003 | Environment separation                 | Accepted |
| 004 | UUIDv7 primary keys                    | Accepted |
| 005 | Opaque server-side sessions            | Accepted |
| 006 | Tracing configured via RUST_LOG        | Accepted |
| 007 | SQLx migrations embedded in the binary | Accepted |
| 008 | Staging environment                    | Proposed |

## Log

### 001. Backend and frontend stack

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Self-hosted chat app; the project is also a way to learn full-stack web development.
- **Decision:** Rust (Axum, Tokio, SQLx) backend; React, TypeScript, Vite frontend; PostgreSQL for data; Garage (S3-compatible) for attachments.
- **Alternatives considered:** _Add if any were seriously weighed._
- **Consequences:** Steeper learning curve (Rust, first-time React) but strong typing across the stack. Rust-specific patterns (error handling, module layout) still need to be decided.

### 002. Single-origin routing

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Cookie-based sessions need simple, secure cookie handling.
- **Decision:** Serve the frontend at `/` and route `/api/*` to the backend under one origin.
- **Alternatives considered:** Separate frontend and API origins (requires CORS and cross-site cookie settings).
- **Consequences:** No CORS needed and `SameSite` cookies just work. Requires a reverse proxy in production and a Vite dev-server proxy in development.

### 003. Environment separation

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** PostgreSQL and Garage instances may be shared across environments.
- **Decision:** Each environment gets its own database, database role, storage bucket, and credentials. Dev never uses production credentials or data.
- **Alternatives considered:** A single shared database/credential set (rejected: a dev mistake could touch production data).
- **Consequences:** More provisioning up front; blast radius of a dev mistake or leaked dev credential is limited to dev.

### 004. UUIDv7 primary keys

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Need non-guessable, globally unique IDs that index well and support cursor pagination for messages.
- **Decision:** `uuid` primary keys using v7, generated in the app with `Uuid::now_v7()`.
- **Alternatives considered:** UUIDv4 (random inserts fragment B-tree indexes and give no time ordering); bigserial (enumerable, leaks counts).
- **Consequences:** IDs embed a creation timestamp (acceptable for these resources). Keep a `created_at` column; don't treat the ID as a precise clock. Session tokens, invite codes, and reset tokens are not UUIDs: use 32 CSPRNG bytes, stored hashed.

### 005. Opaque server-side sessions

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Single-origin web app where the server can look up session state, and immediate revocation matters.
- **Decision:** Random opaque session tokens stored hashed in PostgreSQL, delivered in a cookie. No JWTs.
- **Alternatives considered:** JWTs (harder to revoke, add key and expiry handling with no benefit for a single-origin app).
- **Consequences:** A database lookup per authenticated request. Logout and revoke-all are trivial. Cookie attributes, expiry, and rotation are still to be specified in `security.md`.

### 006. Tracing configured via RUST_LOG

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Need adjustable log verbosity per environment without recompiling.
- **Decision:** Use `tracing` with `tracing-subscriber`'s `EnvFilter`, reading `RUST_LOG` and falling back to `info`. Load `.env` with `dotenvy` via `dotenv().ok()` (not `expect`) so production can use container-provided env vars.
- **Alternatives considered:** `EnvFilter::from_default_env()` alone (defaults to errors only when unset, which looks like missing logs).
- **Consequences:** The subscriber must be initialized after `dotenv()` so `RUST_LOG` in `.env` is honored.

### 007. SQLx migrations embedded in the binary

- **Date:** 2026-11-02
- **Status:** Accepted
- **Context:** Containers should not need migration files at runtime.
- **Decision:** Migrations live in `backend/migrations/`, created with `sqlx migrate add`, embedded via `sqlx::migrate!()`. Applied migrations are never edited; fixes go in new migrations.
- **Alternatives considered:** Shipping SQL files alongside the binary; running only via `sqlx-cli`.
- **Consequences:** Rebuild after adding a migration. Whether production runs migrations on app startup or as a separate step is still open (a bad migration on startup can crash-loop the app).

### 008. Staging environment

- **Date:** 2026-11-02
- **Status:** Proposed
- **Context:** Staging is a production rehearsal (same containers and deploy process, throwaway data). It catches deploy, proxy, and migration problems that local dev can't, but costs extra setup for a solo project.
- **Decision:** Reserve a staging database, role, and bucket in the plan; defer actually standing it up until containerization (milestone 6).
- **Alternatives considered:** Dev and prod only, relying on careful local testing.
- **Consequences:** Revisit at milestone 6. If it becomes overhead, record dropping it here.
