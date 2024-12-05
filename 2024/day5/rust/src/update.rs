use crate::rules::ReachabilityMap;
use std::cmp::Ordering;
use std::convert::identity;

#[derive(Debug)]
pub struct Update {
    pub pages: Vec<u32>,
}

impl Update {
    pub fn is_valid(&self, reach: &ReachabilityMap) -> bool {
        self.pages
            .iter()
            .map_windows(|&[a, b]| reach[a].borrow().contains(b))
            .all(identity)
    }

    pub fn middle_page(&self) -> u32 {
        assert_eq!(self.pages.len() % 2, 1);
        self.pages[self.pages.len() / 2]
    }

    pub fn sorted(&self, reach: &ReachabilityMap) -> Self {
        let mut pages = self.pages.clone();
        pages.sort_by(|a, b| {
            if reach[a].borrow().contains(b) {
                Ordering::Less
            } else if reach[b].borrow().contains(a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        Self { pages }
    }
}
