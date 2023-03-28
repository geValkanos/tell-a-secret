use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Spam {
    hash_map: Arc<Mutex<HashMap<u64, i64>>>,
}

impl Spam {
    pub fn new() -> Self {
        Spam {
            hash_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, id: u64, timestamp: i64) {
        let data = Arc::clone(&self.hash_map);
        let mut dt = data.lock().unwrap();
        (*dt).insert(id, timestamp);
    }

    pub fn get(&self, id: u64) -> Option<i64> {
        let data = Arc::clone(&self.hash_map);
        let dt = data.lock().unwrap();
        (*dt).get(&id).copied()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_update() {
        let test_spam = Spam::new();
        test_spam.insert(1, 1);
        assert_eq!(test_spam.get(1), Some(1));
        test_spam.insert(1, 2);
        assert_eq!(test_spam.get(1), Some(2));
    }

    #[test]
    fn get_not_exists() {
        let test_spam = Spam::new();
        assert_eq!(test_spam.get(1), None);
    }
}
