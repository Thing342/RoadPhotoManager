CREATE TABLE public.PhotoFile
(
  id SERIAL PRIMARY KEY,
  photo_id INT NOT NULL,
  file_size INT NOT NULL,
  file_mime VARCHAR(256) NOT NULL,
  file_uri TEXT NOT NULL,
  photo_width INT NOT NULL,
  photo_height INT NOT NULL,
  CONSTRAINT PhotoFile_photo_id_fk FOREIGN KEY (photo_id) REFERENCES photo (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX PhotoFile_photo_id_index ON public.PhotoFile (photo_id);
COMMENT ON TABLE public.PhotoFile IS 'Representation of a single sizing for a photo. References "photo" many-to-1.';

CREATE TABLE public.PhotoLocation
(
  id SERIAL PRIMARY KEY,
  photo_id INT NOT NULL,
  tm_country VARCHAR(8),
  tm_region VARCHAR(8),
  county VARCHAR(128),
  city VARCHAR(128),
  lat FLOAT,
  lng FLOAT,
  CONSTRAINT PhotoLocation_photo_id_fk FOREIGN KEY (photo_id) REFERENCES photo (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE UNIQUE INDEX PhotoLocation_photo_id_uindex ON public.PhotoLocation (photo_id);
COMMENT ON TABLE public.PhotoLocation IS 'Representation of a location for a photo. References "photo" 1-to-1.';

CREATE TABLE public.PhotoTag
(
  id SERIAL PRIMARY KEY,
  photo_id INT NOT NULL,
  photo_tag VARCHAR(128) NOT NULL,
  note TEXT,
  CONSTRAINT PhotoTag_photo_id_fk FOREIGN KEY (photo_id) REFERENCES photo (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX PhotoTag_photo_id_index ON public.PhotoTag (photo_id);
CREATE INDEX PhotoTag_photo_tag_index ON public.PhotoTag (photo_tag);
COMMENT ON TABLE public.PhotoTag IS 'Represents a tag on a photo, with an optional note. References "photo" many-to-one.';