CREATE TABLE voiceset (
 	id serial PRIMARY KEY,
  	name VARCHAR(80) NOT NULL
);

CREATE TABLE soundset (
  id serial PRIMARY KEY,
  name VARCHAR(80) NOT NULL,
  gender INTEGER CHECK (gender BETWEEN 0 AND 1) NOT NULL,
  soundset_type INTEGER CHECK (soundset_type BETWEEN 0 AND 5) NOT NULL
);

CREATE TABLE sound (
	id serial PRIMARY KEY,
	id_soundset INTEGER REFERENCES soundset(id) ON DELETE CASCADE NOT NULL,
	idx_soundset INTEGER NOT NULL,
	name VARCHAR(80) NOT NULL,
	transcription VARCHAR(400),
	sound_data bytea NOT NULL,
  	UNIQUE(id_soundset, idx_soundset)
);

CREATE TABLE voiceset_soundset (
  id serial PRIMARY KEY,
  id_voiceset INTEGER REFERENCES voiceset(id) ON DELETE CASCADE NOT NULL,
  id_soundset INTEGER REFERENCES soundset(id) ON DELETE CASCADE NOT NULL,
  UNIQUE(id_voiceset, id_soundset)
);