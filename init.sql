CREATE TABLE test(
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name TEXT NOT NULL,
);

-- add test data
INSERT INTO test (name)
  VALUES ('test');
