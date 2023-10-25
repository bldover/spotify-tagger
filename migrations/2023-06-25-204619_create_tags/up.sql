CREATE TABLE IF NOT EXISTS tags
(
    tag_id INTEGER GENERATED ALWAYS AS IDENTITY,
    tag_name text COLLATE pg_catalog."default" NOT NULL UNIQUE,
    CONSTRAINT tags_pkey PRIMARY KEY (tag_id)
    INCLUDE(tag_id)
)