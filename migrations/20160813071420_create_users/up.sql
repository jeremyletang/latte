CREATE TABLE users (
  id TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  token_id TEXT NOT NULL,
  slack_user_id TEXT NOT NULL
)
