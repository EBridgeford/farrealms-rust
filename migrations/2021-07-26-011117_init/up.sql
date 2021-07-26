CREATE TABLE Users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  pass VARCHAR NOT NULL,
  create_date TIMESTAMP WITH TIME ZONE NOT NULL default (now() at time zone 'utc')
);

CREATE TABLE Posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  post TEXT NOT NULL,
  author SERIAL REFERENCES Users (id),
  create_date  TIMESTAMP WITH TIME ZONE NOT NULL,
  update_date  TIMESTAMP WITH TIME ZONE NOT NULL
);

INSERT INTO Users(username, email, pass, create_date) VALUES('Eric', 'eric@mail.com', 'hunter2', timezone('utc', now()));
INSERT INTO Users(username, email, pass, create_date) VALUES('Killer Bob', 'blacklodge@mail.com', 'hunter3', timezone('utc', now()));