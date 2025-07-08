use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Metadata {
    pub freq: u64,
    // NOTE: second since the Unix epoch 
    pub created_at: u64
}

impl Metadata {
    pub fn new() -> Metadata {
        let currtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cant get the current time")
            .as_secs();
        Metadata { 
            freq: 0,
            created_at: currtime 
        }
    }

    pub fn freq_incretement(&mut self) {
        self.freq += 1
    }

    pub fn freq_decretement(&mut self) {
        self.freq -= 1
    }
}
