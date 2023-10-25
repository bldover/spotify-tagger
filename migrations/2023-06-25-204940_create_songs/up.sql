CREATE TABLE IF NOT EXISTS songs
(
    song_id INTEGER GENERATED ALWAYS AS IDENTITY,
    song_uri text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT songs_pkey PRIMARY KEY (song_id)
    INCLUDE(song_id)
)