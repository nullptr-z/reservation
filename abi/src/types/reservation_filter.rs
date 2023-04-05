use std::collections::VecDeque;

use crate::{
    Error, FilterPager, Normalizer, Reservation, ReservationFilter, ReservationStatus, ToSql,
    Validate,
};

impl ReservationFilter {
    pub fn get_cursor(&self) -> i64 {
        match self.cursor {
            Some(v) => v,
            None => {
                if self.desc {
                    i64::MAX
                } else {
                    0
                }
            }
        }
    }

    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::from_i32(self.status).unwrap()
    }

    pub fn get_desc_str(&self) -> &str {
        if self.desc {
            "DESC"
        } else {
            "ASC"
        }
    }

    pub fn get_pager(&self, data: &VecDeque<Reservation>) -> Result<FilterPager, Error> {
        let has_prev = self.cursor.is_some();
        let start = if has_prev { data.pop_front() } else { None };

        let has_next = data.len() as i64 > self.page_size;
        let end = if has_next { data.pop_back() } else { None };

        let pager = FilterPager {
            prev: start.map(|r| r.id),
            next: end.map(|r| r.id),
            // TODO: how to get total efficiently?
            total: 0,
        };

        Ok(pager)
    }
}

impl Validate for ReservationFilter {
    fn validate(&self) -> Result<(), crate::Error> {
        if self.page_size < 10 || self.page_size > 100 {
            return Err(Error::InvalidPageSize(self.page_size));
        }

        if let Some(Cursor::Value(cursor)) = self.cursor {
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
        let middle_puls = if self.cursor.is_none() { 0 } else { 1 };

        let mut sql = "SELECT * FROM rsvp.reservation WHERE ".to_string();
        let compare = match self.desc {
            true => "<=",
            false => ">=",
        };
        sql.push_str(&format!("id {} {} AND ", compare, self.get_cursor()));
        sql.push_str(&format!("status = {} AND ", self.get_status()));
        if self.user_id.is_empty() && self.resource_id.is_empty() {
            sql.push_str("TRUE");
        } else if self.user_id.is_empty() {
            sql.push_str(&format!("resource_id = '{}'", self.resource_id));
        } else if self.resource_id.is_empty() {
            sql.push_str(&format!("user_id = '{}'", self.user_id));
        } else {
            sql.push_str(&format!(
                "resource_id = '{}' AND user_id = '{}",
                self.resource_id, self.user_id
            ));
        }
        sql.push_str(&format!(
            " ORDER BY id {} LIMIT {}",
            self.get_desc_str(),
            self.page_size + 1 + middle_puls
        ));
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReservationFilterBuilder;

    #[test]
    fn filter_should_generate_correct_sql() {
        let filter = ReservationFilterBuilder::default()
            .user_id("zz id")
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE id <= 0 AND status = 1 AND user_id = 'zz id' AND TRUE ORDER BY id ASC LIMIT 10");

        let filter = ReservationFilterBuilder::default()
            .user_id("try")
            .resource_id("test")
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 1 AND id >= 0 AND user_id = 'zz id' AND resource_id = 'test' ORDER BY id ASC LIMIT 10");

        let filter = ReservationFilterBuilder::default()
            .desc(true)
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservation WHERE status = 1 AND id >= 0 ORDER BY id ASC LIMIT 10"
        );

        let filter = ReservationFilterBuilder::default()
            .user_id("try")
            .cursor(Some(Cursor::Value(100)))
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 1 AND id >= 100 AND user_id = 'zz id' AND TRUE ORDER BY id ASC LIMIT 10");

        let filter = ReservationFilterBuilder::default()
            .user_id("try")
            .cursor(Some(Cursor::Value(10)))
            .desc(true)
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 1 AND id <= 10 AND user_id = 'zz id' AND TRUE ORDER BY id DESC LIMIT 10");
    }
}
