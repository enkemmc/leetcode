struct  MyHashSet {
    set: Vec<bool>,
    capacity: usize
}

impl MyHashSet {
    fn new() -> Self {
        let capacity = 1000;
        Self {
            set: vec![false; capacity],
            capacity
        }
    }

    fn add(&mut self, key: i32) {
        // if its oob, grow capacity
        if key as usize > self.capacity {
            self.grow(key);
        }
        self.set[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        if self.capacity < key as usize {
            self.grow(key);
        }
        self.set[key as usize] = false;
    }

    fn contains(&mut self, key: i32) -> bool {
        if self.capacity < key as usize {
            self.grow(key);
        }
        self.set[key as usize]
    }

    fn grow(&mut self, key: i32) {
        let mut new = (self.capacity..=key as usize)
            .map(|_n| false)
            .collect();
        self.set.append(&mut new);
        self.capacity = key as usize;
    }
}
