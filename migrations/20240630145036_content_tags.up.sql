CREATE TABLE content_tags (
	id int4 GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1 NO CYCLE) NOT NULL,
	content_id int4 NOT NULL,
	tag_id int4 NOT NULL,
	created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
	CONSTRAINT content_tags_pkey PRIMARY KEY (id),
	CONSTRAINT fk_content FOREIGN KEY (content_id) REFERENCES "content"(id) ON DELETE CASCADE,
	CONSTRAINT fk_tags FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE RESTRICT
);

CREATE INDEX idx_content_tags_tag_id ON public.content_tags USING btree (tag_id);
