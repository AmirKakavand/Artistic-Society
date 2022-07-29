CREATE TABLE artist(
	id serial PRIMARY KEY,
	name text NOT NULL,
	birthdate date
);

CREATE TABLE song(
	id serial PRIMARY KEY,
	name text NOT NULL,
	location text NOT NULL,
	artist integer NOT NULL,
	duration integer NOT NULL,
	genre text,

	UNIQUE(location),
	FOREIGN KEY (artist) REFERENCES artist (id)
);

CREATE TABLE image(
	id serial PRIMARY KEY,
	name text,
	location text NOT NULL,
	artist integer,

	UNIQUE(location),
	FOREIGN KEY (artist) REFERENCES artist (id)
);
