use std::collections::VecDeque;

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
        let start = if has_prev { data.pop_front() } else { None };

        let has_next = data.len() as i64 > self.page_size;
        let end = if has_next { data.pop_back() } else { None };

        let pager = Pager {
            prev: start.map(|r| r.id()),
            next: end.map(|r| r.id()),
            // TODO: how to get total efficiently?
            total: None,
        };

        pager
    }

    fn next_page(&self, pager: &Pager) -> Option<Self> {
        if pager.next.is_none() {
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
        if pager.next.is_none() {
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

#[test]
fn paginator_should_work() {
    pub struct TestId(i64);

    // first page
    let page = PageInfo {
        cursor: None,
        page_size: 10,
        desc: false,
    };

    // assume we got 11 items
    // create 100 items
    let mut items: VecDeque<TestId> = (1..=11).iter().map(|i| TestId(*i)).collect();
    let pager = page.get_pager(&mut items);

    assert!(pager.prev.is_none());
}
