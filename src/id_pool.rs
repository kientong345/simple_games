use std::collections::BTreeSet;

pub struct IdPool {
    pool: BTreeSet<i32>,
}

impl IdPool {
    pub fn new() -> Self {
        Self {
            pool: BTreeSet::new(),
        }
    }

    pub fn alloc_id(&mut self) -> i32 {
        let mut cnt = 1;
        let new_id = loop {
            if !self.pool.contains(&cnt) {
                break cnt
            }
            cnt += 1;
        };
        self.pool.insert(new_id);
        new_id
    }

    pub fn dealloc_id(&mut self, id: i32) -> bool {
        self.pool.remove(&id)
    }
}