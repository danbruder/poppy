-- Add migration script here
CREATE TABLE org (
  id TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL, 

  created_at TEXT,
  updated_at TEXT
);

CREATE TABLE user (
  id TEXT NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL, 
  last_name TEXT NOT NULL, 
  email TEXT NOT NULL, 
  org_id TEXT NOT NULL,

  created_at TEXT,
  updated_at TEXT
);
