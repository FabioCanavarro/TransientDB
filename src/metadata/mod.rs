use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Metadata {
    freq: u64,
    // NOTE: second since the Unix epoch 
    created_at: u64
}

impl Metadata {
    fn new() -> Metadata {
        let currtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cant get the current time")
            .as_secs();
        Metadata { 
            freq: 0,
            created_at: currtime 
        }
    }
}
