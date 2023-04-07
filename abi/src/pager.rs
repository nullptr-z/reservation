use std::collections::VecDeque;

#[derive(Debug)]
pub struct Pager {
    pub prev: Option<i64>,
    pub next: Option<i64>,
    pub total: Option<i64>,
}

pub struct PageInfo {
    pub cursor: Option<i64>,
    pub page_size: i64,
    pub desc: bool,
}

pub trait Paginator: Sized {
    fn get_pager<T: Id>(&self, data: &mut VecDeque<T>) -> Pager;
    fn next_page(&self, pager: &Pager) -> Option<Self>;
    fn prev_page(&self, pager: &Pager) -> Option<Self>;
}

pub trait Id {
    fn id(&self) -> i64;
}

impl Paginator for PageInfo {
    fn get_pager<T: Id>(&self, data: &mut VecDeque<T>) -> Pager {
        let has_prev = self.cursor.is_some();
        let prev = if has_prev {
            data.pop_front();
            data.front().map(|v| v.id())
        } else {
            None
        };

        let has_next = data.len() as i64 > self.page_size;
        let next = if has_next {
            data.pop_back();
            data.back().map(|v| v.id())
        } else {
            None
        };

        let pager = Pager {
            prev,
            next,
            // TODO: how to get total efficiently?
            total: None,
        };

        pager
    }

    fn next_page(&self, pager: &Pager) -> Option<Self> {
        if pager.next.is_some() {
            Some(PageInfo {
                cursor: pager.next,
                page_size: self.page_size,
                desc: self.desc,
            })
        } else {
            None
        }
    }

    fn prev_page(&self, pager: &Pager) -> Option<Self> {
        if pager.next.is_some() {
            Some(PageInfo {
                cursor: pager.prev,
                page_size: self.page_size,
                desc: self.desc,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestId(i64);

    impl Id for TestId {
        fn id(&self) -> i64 {
            self.0
        }
    }

    #[test]
    fn paginator_should_work() {
        // first page
        let page = PageInfo {
            cursor: None,
            page_size: 10,
            desc: false,
        };

        // assume we got 11 items
        // create 100 items
        let mut items: VecDeque<TestId> = (1..=11).map(|i| TestId(i)).collect();
        let pager = page.get_pager(&mut items);
        assert!(pager.prev.is_none());
        assert_eq!(pager.next, Some(10));

        // second page
        let page = page.next_page(&pager).unwrap();
        let mut items: VecDeque<TestId> = (10..=21).map(|i| TestId(i)).collect();
        let pager = page.get_pager(&mut items);
        assert_eq!(pager.prev, Some(11));
        assert_eq!(pager.next, Some(20));

        // third page
        let page = page.next_page(&pager).unwrap();
        let mut items: VecDeque<TestId> = (20..=25).map(|i| TestId(i)).collect();
        let pager = page.get_pager(&mut items);
        assert_eq!(pager.prev, Some(21));
        assert!(pager.next.is_none());
    }
}
