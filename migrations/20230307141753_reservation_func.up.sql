-- 如果用户id为空，则查找资源的所有预定
-- 如果资源id为空，则查找用户的所有预定
-- 如果两者都为空，则查找所有预定
-- 如果两者都设置了，则查所有预定和用户
CREATE OR REPLACE FUNCTION rsvp.query(
  uid text,
  rid text,
  during TSTZRANGE,
  status rsvp.reservation_status,
  is_desc bool DEFAULT FALSE,
  page integer DEFAULT 1,
  page_size integer DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservation) AS $$
DECLARE
  _sql text;
BEGIN
  -- format the query based on parameters`根据参数格式化查询
  _sql := format(
    'SELECT * FROM rsvp.reservation WHERE %L @> timespan AND status = %L AND %s ORDER BY lower(timespan) %s LIMIT %L::integer OFFSET %L::integer',
    during,
    status,
    CASE
      WHEN uid IS NULL AND rid IS NULL THEN 'TRUE'
      WHEN uid IS NULL THEN 'resource_id = ' || quote_literal(rid)
      WHEN rid IS NULL THEN 'user_id = ' || quote_literal(uid)
      ELSE 'user_id = ' || quote_literal(uid) || 'AND resource_id = ' || quote_literal(rid)
    END,
    CASE
      WHEN is_desc THEN 'DESC'
      ELSE 'ASC'
    END,
    page_size,
    (page - 1) * page_size
  );

  -- execute the query
  RETURN QUERY EXECUTE _sql;

END;
$$ LANGUAGE plpgsql;
