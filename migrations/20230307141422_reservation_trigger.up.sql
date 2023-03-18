-- reservation change queue
CREATE TABLE rsvp.reservation_changes (
  id SERIAL NOT NULL,
  reservation_id BIGSERIAL NOT NULL,
  op rsvp.reservation_update_type NOT NULL
);

-- tergeer`触发 for add/update/delete a reservation
CREATE OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS $$
BEGIN
  IF TG_OP = 'INSERT' THEN
    INSERT INTO rsvp.reservation_changes(reservation_id, op) VALUES (NEW.id, 'create');
  ELSEIF TG_OP = 'UPDATE' THEN
    -- 如果OLD.status 不等于 NEW.status，改变 reservation_changes
    IF OLD.status <> NEW.status THEN
      INSERT INTO rsvp.reservation_changes(reservation_id, op) VALUES (NEW.id, 'update');
    END IF;
  ELSEIF TG_OP = 'DELETE' THEN
    INSERT INTO rsvp.reservation_changes(reservation_id, op) VALUES (OLD.id, 'delete');
  END IF;

  NOTIFY reservation_update;
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reservation_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservation
    FOR EACH ROW EXECUTE PROCEDURE rsvp.reservation_trigger();



