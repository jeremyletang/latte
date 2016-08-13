CREATE TABLE messages (
  id TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  user_id TEXT NOT NULL,
  body TEXT NOT NULL,
  channel TEXT NOT NULL,
  at INTEGER NOT NULL,
  weekdays TEXT NOT NULL,
  repeated INTEGER
);
