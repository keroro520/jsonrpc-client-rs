use jsonrpc_core::types::Id;
use std::sync::Mutex;

#[derive(Debug)]
pub struct IdGenerator {
    next_id: Mutex<u64>,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator { next_id: Mutex::new(1), }
    }

    pub fn next(&self) -> Id {
        Id::Num(self.next_int())
    }

    pub fn next_int(&self) -> u64 {
        let mut id = self.next_id.lock().expect("acquire id");
        let next_id = *id;
        *id += 1;
        next_id
    }
}
