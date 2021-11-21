-- Your SQL goes here
CREATE TABLE todos (
  id UUID NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  is_complete BOOL NOT NULL,
  owner VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL
)
