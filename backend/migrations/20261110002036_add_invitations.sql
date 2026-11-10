CREATE TABLE invitations (
    id          uuid PRIMARY KEY,
    code_hash   bytea       NOT NULL UNIQUE,
    created_by  uuid        REFERENCES users(id),  -- NULL for bootstrap invites
    created_at  timestamptz NOT NULL DEFAULT now(),
    expires_at  timestamptz NOT NULL,
    used_by     uuid        REFERENCES users(id),
    used_at     timestamptz,
    CHECK ((used_by IS NULL) = (used_at IS NULL))
);
