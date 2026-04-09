CREATE TABLE IF NOT EXISTS identity.users (
      id            UUID        PRIMARY KEY,
      email         TEXT        NOT NULL UNIQUE,
      password_hash TEXT        NOT NULL,
      role          TEXT        NOT NULL CHECK (role IN ('Maker', 'Buyer', 'Admin')),
      status        TEXT        NOT NULL CHECK (status IN ('PendingValidation', 'Active')),
      created_at    TIMESTAMPTZ NOT NULL
  );

  CREATE UNIQUE INDEX IF NOT EXISTS idx_identity_users_email ON identity.users(email);