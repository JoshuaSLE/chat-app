# API

> Status: planning. Endpoints are proposals until implemented.

## Conventions

- Base path: `/api` (same origin as the frontend).
- Format: JSON, UTF-8.
- Auth: session cookie. Details in [security.md](security.md).
- IDs: UUID strings (v7).
- Timestamps: RFC 3339 / ISO 8601 in UTC.
- Versioning: _TBD (e.g. `/api/v1`)._
- Pagination: cursor-based on `id` (`?before=<uuid>&limit=50`). (Proposed)

## Error format

_TBD. Proposed shape:_

```json
{
  "error": {
    "code": "invalid_credentials",
    "message": "Invalid username or password"
  }
}
```

| Status | Meaning                                          |
| ------ | ------------------------------------------------ |
| 400    | Malformed request / validation failure           |
| 401    | Not authenticated                                |
| 403    | Authenticated but not allowed                    |
| 404    | Not found (also used to avoid leaking existence) |
| 409    | Conflict (e.g. username taken)                   |
| 429    | Rate limited                                     |
| 500    | Server error                                     |

## Endpoints

### Health

| Method | Path          | Auth | Description    |
| ------ | ------------- | ---- | -------------- |
| GET    | `/api/health` | no   | Liveness check |

### Auth (milestone 3)

| Method | Path                 | Auth             | Description    |
| ------ | -------------------- | ---------------- | -------------- |
| POST   | `/api/auth/register` | no (invite code) | Create account |
| POST   | `/api/auth/login`    | no               | Start session  |
| POST   | `/api/auth/logout`   | yes              | End session    |
| GET    | `/api/auth/me`       | yes              | Current user   |

### Invitations (milestone 3)

_TBD._

### Conversations and messages (milestone 4)

_TBD: list/create conversations, manage members, list/send messages._

### Attachments (milestone 5)

_TBD._

### WebSocket (milestone 5)

_TBD: endpoint, handshake, event types (message, typing, presence)._

## Endpoint detail template

Copy per endpoint once implemented:

```text
### METHOD /api/path

Auth: required | none
Description: ...

Request:
  (JSON body / query params)

Response 2xx:
  (JSON)

Errors:
  (status: code, when)
```
