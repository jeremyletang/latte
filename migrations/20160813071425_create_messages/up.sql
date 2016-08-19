CREATE TABLE messages (
  id TEXT PRIMARY KEY,
  created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
  updated_at INTEGER DEFAULT CURRENT_TIMESTAMP,

  user_id TEXT,
  body TEXT NOT NULL,
  channel TEXT NOT NULL,

  seconds INTEGER NOT NULL,
  utc_offset INTEGER NOT NULL,

  monday INTEGER,
  tuesday INTEGER,
  wednesday INTEGER,
  thursday INTEGER,
  friday INTEGER,
  saturday INTEGER,
  sunday INTEGER,
  repeated INTEGER NOT NULL
);
