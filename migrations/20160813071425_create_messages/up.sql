CREATE TABLE messages (
  id TEXT PRIMARY KEY,
  created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
  updated_at INTEGER DEFAULT CURRENT_TIMESTAMP,

  user_id TEXT,
  body TEXT NOT NULL,
  channel TEXT NOT NULL,

  seconds INTEGER NOT NULL,
  utc_offset INTEGER NOT NULL,

  weekdays_id TEXT NOT NULL,
  repeated INTEGER NOT NULL
);
