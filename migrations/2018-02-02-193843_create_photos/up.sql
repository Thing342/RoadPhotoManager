CREATE TABLE public.Photo
(
  id SERIAL PRIMARY KEY,
  photo_type VARCHAR(16) DEFAULT 'UNCAT' NOT NULL,
  uploaded_at TIMESTAMP DEFAULT NOW() NOT NULL,
  caption TEXT DEFAULT '' NOT NULL,
  alt_text TEXT DEFAULT '' NOT NULL
);
CREATE INDEX Photo_photo_type_index ON public.Photo (photo_type);
COMMENT ON TABLE public.Photo IS 'Base table for photos.'