use crate::{Error, Normalizer, ReservationFilter, ReservationStatus, ToSql, Validate};

impl Validate for ReservationFilter {
    fn validate(&self) -> Result<(), crate::Error> {
        if self.page_size < 10 || self.page_size > 100 {
            return Err(Error::InvalidPageSize(self.page_size));
        }

        if let cursor = self.cursor {
            if cursor < 0 {
                return Err(Error::InvalidCursor(cursor));
            }
        }

        if self.status <= ReservationStatus::Unknown as i32 {
            return Err(Error::InvalidStatus(self.status));
        }

        ReservationStatus::from_i32(self.status).ok_or(Error::InvalidStatus(self.status))?;

        Ok(())
    }
}

impl Normalizer for ReservationFilter {
    fn do_nomalize(&mut self) {
        todo!()
    }
}

/**
   *   -- if page_size is not between 10 an 100, set ii to 10
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
   */
impl ToSql for ReservationFilter {
    fn to_sql(&self) -> Result<String, Error> {
        todo!()
    }
}
