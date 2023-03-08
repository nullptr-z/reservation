-- 如果用户id为空，则查找资源的所有预定
-- 如果资源id为空，则查找用户的所有预定
-- 如果两者都为空，则查找所有预定
-- 如果两者都设置了，则查所有预定和用户
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during TSTZRANGE) RETURNS TABLE (LIKE rsvp.reservation)
AS $$
BEGIN
  IF uid IS NULL AND rid IS NULL THEN
    RETURN QUERY SELECT * FROM rsvp.reservation WHERE timespan && during;
  ELSEIF uid IS NULL THEN
    RETURN QUERY SELECT * FROM rsvp.reservation WHERE resource_id == rid AND during @> timespan;
  ELSEIF rid IS NULL THEN
    RETURN QUERY SELECT * FROM rsvp.reservation WHERE user_id == uid AND during @> timespan;
  ELSE
    RETURN QUERY SELECT * FROM rsvp.reservation WHERE user_id == uid AND  resource_id == rid AND during @> timespan;
  END IF;
END
$$ LANGUAGE plpgsql;
