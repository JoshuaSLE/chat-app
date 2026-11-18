CREATE TABLE sessions (
  token_hash BYTEA PRIMARY KEY,          -- SHA-256 of the token, never the raw token
  user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX ON sessions (user_id);
