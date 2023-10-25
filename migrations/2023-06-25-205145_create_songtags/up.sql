CREATE TABLE IF NOT EXISTS song_tags
(
    song_id integer NOT NULL,
    tag_id integer NOT NULL,
    CONSTRAINT song_tags_pkey PRIMARY KEY (song_id, tag_id),
    CONSTRAINT "SongFK" FOREIGN KEY (song_id)
    REFERENCES songs (song_id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION,
    CONSTRAINT "TagFK" FOREIGN KEY (tag_id)
    REFERENCES tags (tag_id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
)