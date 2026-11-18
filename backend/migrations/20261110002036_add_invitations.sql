CREATE TABLE invitations (
    id          UUID PRIMARY KEY,
    code_hash   BYTEA       NOT NULL UNIQUE,
    created_by  UUID        REFERENCES users(id) ON DELETE SET NULL,  -- NULL for bootstrap invites
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at  TIMESTAMPTZ NOT NULL,
    used_by     UUID        REFERENCES users(id) ON DELETE CASCADE,
    used_at     TIMESTAMPTZ,
    CHECK ((used_by IS NULL) = (used_at IS NULL))
);
