-- Your SQL goes here
CREATE TABLE "user" (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  "password" VARCHAR NOT NULL,
  is_superuser BOOLEAN NOT NULL DEFAULT 'f'
)