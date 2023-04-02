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
  page_size bigint DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservation) AS $$
DECLARE
  _sql text;
BEGIN
  -- if page is less than 1, set it to 1
  IF page < 1 THEN
    page := 1;
  END IF;

  -- if page_size is not between 10 an 100, set ii to 10
  IF page_size < 10 OR page_size > 100 THEN
    page_size := 10;
  END IF;

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


-- Filter Reservation --------------------------------------------------------

-- we filter 2 more items on for starting. one for ending.
-- 我们过滤 2 个以上的项目作为开始。 一个结束。
-- if starting existing, then we have previous page,
-- 如果从现有开始，那么我们有上一页，
-- if ending existing, then we have next page
-- 如果现有结束，那么我们有下一页
CREATE OR REPLACE FUNCTION rsvp.filter(
  uid text,
  rid text,
  status rsvp.reservation_status,
  cursor bigint DEFAULT NULL,
  is_desc bool DEFAULT FALSE,
  page_size bigint DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservation) AS $$
DECLARE
  _sql text;
  _offset bigint;
BEGIN

  -- if page_size is not between 10 an 100, set ii to 10
  IF page_size < 10 OR page_size > 100 THEN
    page_size := 10;
  END IF;

  -- if page is less than 1, set it to 1
  IF cursor IS NULL OR cursor < 0 THEN
    IF is_desc THEN
      cursor := 2^64 - 1;
    ELSE
      cursor :=0;
    END IF;
  END IF;

  -- format the query based on parameters`根据参数格式化查询
  _sql := format(
    'SELECT * FROM rsvp.reservation WHERE %s AND status = %L AND %s ORDER BY id %s LIMIT %L::integer',
    CASE
      WHEN is_desc THEN 'id <= ' || cursor
      ELSE 'id >= ' || cursor
    END,
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
    page_size + 1
  );

  -- execute the query
  RETURN QUERY EXECUTE _sql;

END;
$$ LANGUAGE plpgsql;
