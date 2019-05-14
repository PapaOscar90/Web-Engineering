CREATE TABLE carriers (
  id SERIAL PRIMARY KEY NOT NULL,
  code CHAR(2) NOT NULL,
  name STRING NOT NULL,
  CONSTRAINT code_name UNIQUE (code, name)
);
