use std::collections::VecDeque;

use crate::{
    pager::{Id, PageInfo, Pager, Paginator},
    Error, FilterPager, Normalizer, Reservation, ReservationFilter, ReservationFilterBuilder,
    ReservationStatus, ToSql, Validate,
};

impl ReservationFilterBuilder {
    pub fn build(&self) -> Result<ReservationFilter, Error> {
        let mut filter = self
            .private_build()
            .expect("failed to build ReservationFilter");
        filter.normalize()?;

        Ok(filter)
    }
}

impl From<Pager> for FilterPager {
    fn from(pager: Pager) -> Self {
        Self {
            prev: pager.prev,
            next: pager.next,
            total: pager.total,
        }
    }
}

impl From<&FilterPager> for Pager {
    fn from(pager: &FilterPager) -> Self {
        Self {
            prev: pager.prev,
            next: pager.next,
            total: pager.total,
        }
    }
}

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

    pub fn get_pager<T: Id>(&self, data: &mut VecDeque<T>) -> FilterPager {
        let page_info = self.page_info();
        let pager = page_info.get_pager(data);

        pager.into()
    }

    fn page_info(&self) -> PageInfo {
        PageInfo {
            cursor: self.cursor,
            page_size: self.page_size,
            desc: self.desc,
        }
    }

    pub fn next_page(&self, pager: &FilterPager) -> Option<Self> {
        let page_info = self.page_info();
        let pager = pager.into();
        let page_info = page_info.next_page(&pager);
        page_info.map(|page| Self {
            cursor: page.cursor,
            page_size: page.page_size,
            desc: page.desc,
            resource_id: self.resource_id.clone(),
            user_id: self.user_id.clone(),
            status: self.status,
        })
    }
}

impl Validate for ReservationFilter {
    fn validate(&self) -> Result<(), crate::Error> {
        if self.page_size < 10 || self.page_size > 100 {
            return Err(Error::InvalidPageSize(self.page_size));
        }

        if let Some(cursor) = self.cursor {
            if cursor < 0 {
                return Err(Error::InvalidCursor(cursor));
            }
        }

        ReservationStatus::from_i32(self.status).ok_or(Error::InvalidStatus(self.status))?;

        Ok(())
    }
}

impl Normalizer for ReservationFilter {
    fn do_nomalize(&mut self) {
        if self.status == ReservationStatus::Unknown as i32 {
            self.status = ReservationStatus::Pending as i32;
        }
    }
}

impl ToSql for ReservationFilter {
    fn to_sql(&self) -> Result<String, Error> {
        let middle_puls = if self.cursor.is_none() { 0 } else { 1 };

        let compare = match self.desc {
            true => "<=",
            false => ">=",
        };

        let uid_rid_cond = match (self.user_id.is_empty(), self.resource_id.is_empty()) {
            (true, true) => "TRUE".to_string(),
            (false, true) => format!("user_id = '{}'", self.user_id),
            (true, false) => format!("resource_id = '{}'", self.resource_id),
            _ => format!(
                "user_id = '{}' AND resource_id = '{}'",
                self.user_id, self.resource_id
            ),
        };

        let direction = if self.desc { "DESC" } else { "ASC" };

        let sql = format!(
            "SELECT * FROM rsvp.reservation WHERE status = '{}'::rsvp.reservation_status AND id {} {} AND {} ORDER BY id {} LIMIT {}",
            self.get_status().to_string(),
            compare,
            self.get_cursor(),
            uid_rid_cond,
            direction,
            self.page_size + 1 + middle_puls
        );

        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pager::pager_test_utils::generate_test_ids, ReservationFilterBuilder};

    #[test]
    // fn filter_should_generate_correct_sqls() {
    //     todo!()
    // }
    #[test]
    fn filter_should_generate_correct_sql() {
        let filter = ReservationFilterBuilder::default()
            .user_id("zz id")
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id >= 0 AND user_id = 'zz id' ORDER BY id ASC LIMIT 11");

        let filter = ReservationFilterBuilder::default()
            .user_id("zz id")
            .resource_id("test")
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id >= 0 AND user_id = 'zz id' AND resource_id = 'test' ORDER BY id ASC LIMIT 11");

        let filter = ReservationFilterBuilder::default()
            .desc(true)
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id <= 9223372036854775807 AND TRUE ORDER BY id DESC LIMIT 11"
        );

        let filter = ReservationFilterBuilder::default()
            .user_id("zz id")
            .cursor(Some(100))
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,"SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id >= 100 AND user_id = 'zz id' ORDER BY id ASC LIMIT 12");

        let filter = ReservationFilterBuilder::default()
            .user_id("zz id")
            .cursor(Some(10))
            .desc(true)
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql,
            "SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id <= 10 AND user_id = 'zz id' ORDER BY id DESC LIMIT 12"
            );
    }

    #[test]
    fn filter_with_pager_should_generate_correct_sql() {
        let filter = ReservationFilterBuilder::default()
            .resource_id("zz id")
            .build()
            .unwrap();
        let mut data = generate_test_ids(1..=11);
        let pager = filter.get_pager(&mut data);
        assert_eq!(pager.prev, None);
        assert_eq!(pager.next, Some(10));

        let filter = filter.next_page(&pager).unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(sql, "SELECT * FROM rsvp.reservation WHERE status = 'pending'::rsvp.reservation_status AND id >= 10 AND resource_id = 'zz id' ORDER BY id ASC LIMIT 12");
        let mut data = generate_test_ids(10..=20);
        let pager = filter.get_pager(&mut data);
        assert_eq!(pager.prev, Some(11));
        assert_eq!(pager.next, None);
    }
}
