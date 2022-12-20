-- Create table for ExamplePost endpoint
CREATE TABLE example(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   added_at timestamptz NOT NULL
);