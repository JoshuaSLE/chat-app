# Database

> Status: planning. The schema below is a draft until its migration is applied.

## Conventions

- PostgreSQL, accessed through SQLx.
- **Primary keys:** `uuid`, UUIDv7, generated in the app (`Uuid::now_v7()`). See [decisions.md](decisions.md).
- **Timestamps:** `timestamptz`, `created_at NOT NULL DEFAULT now()`. Don't rely on UUID ordering as a precise clock.
- **Naming:** `snake_case`, plural table names.
- **Case-insensitive uniqueness** (username, email) via unique index on `lower(column)`.
- **Secrets** (session tokens, invite codes) are stored hashed, never raw.

## Migrations

- Location: `backend/migrations/`; created with `sqlx migrate add <name>`.
- Applied with `sqlx migrate run`; embedded into the binary via `sqlx::migrate!()`.
- Never edit an applied migration; add a new one.
- One concern per migration, descriptive names.
- Production: _TBD (run on startup vs separate step)._

## Environments

| Environment | Database       | Role                            |
| ----------- | -------------- | ------------------------------- |
| dev         | `chat_app_dev` | `chat_app_dev` (database owner) |
| staging     | _TBD_          | _TBD_                           |
| prod        | _TBD_          | _TBD_                           |

Provisioning (run as admin, once per environment):

```sql
CREATE ROLE chat_app_dev LOGIN PASSWORD '<strong-password>';
CREATE DATABASE chat_app_dev OWNER chat_app_dev;
```

## Schema

### users (draft)

| Column        | Type                 | Notes                       |
| ------------- | -------------------- | --------------------------- |
| id            | uuid PK              | UUIDv7, app-generated       |
| username      | text NOT NULL        | unique on `lower(username)` |
| email         | text NOT NULL        | unique on `lower(email)`    |
| password_hash | text NOT NULL        | Argon2id (PHC string)       |
| created_at    | timestamptz NOT NULL | default `now()`             |

### invitations

| Column     | Type                 | Notes                          |
| ---------- | -------------------- | ------------------------------ |
| id         | uuid PK              | UUIDv7, app-generated          |
| code_hash  | bytea                | SHA256 hash of invitation code |
| created_by | uuid                 | UUIDv7, app-generated          |
| created_at | timestamptz NOT NULL | default `now()`                |
| expires_at | timestamptz NOT NULL | `now() + 7 days` app-generated |
| used_by    | uuid                 | UUIDv7, app-generated          |
| used_at    | timestamptz          |                                |

### sessions

_TBD (milestone 3): hashed token, user_id, created/expires/last_seen, revocation._

### conversations / memberships

_TBD (milestone 4): direct vs group, members, roles._

### messages

_TBD (milestone 4): conversation_id, sender_id, body, timestamps, pagination index._

### attachments

_TBD (milestone 5): object key, content type, size, owner._

## Entity relationships

_TBD diagram._

## Indexing and performance notes

- _Add as queries are written._
