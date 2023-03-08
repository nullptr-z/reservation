-- Add up migration script here

CREATE TYPE rsvp.reservation_status AS ENUM('unknown','pending','confirmed','blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM('unknown','create','update','delete');
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE rsvp.reservation (
  id uuid NOT NULL DEFAULT gen_random_uuid(),
  user_id VARCHAR(64) NOT NULL,
  status rsvp.reservation_status NOT NULL DEFAULT 'pending',

  resource_id VARCHAR(64) NOT NULL,
  timespan TSTZRANGE NOT NULL,

  note TEXT,

  CONSTRAINT raservation_pkey PRIMARY KEY (id),

  CONSTRAINT raservation_conflict EXCLUDE USING gist (resource_id WITH=, timespan WITH && )
);

CREATE INDEX reservation_resource_id_idx ON rsvp.reservation (resource_id);
CREATE INDEX reservation_user_id_idx ON rsvp.reservation (user_id);
