use std::time::{SystemTime, UNIX_EPOCH};

use bincode::{error::{DecodeError, EncodeError}, serde::{decode_from_slice, encode_to_vec}};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
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

    pub fn to_u8(&self) -> Result<Vec<u8>, EncodeError>{
        encode_to_vec(
            self,
            bincode::config::standard()
        )
    }

    pub fn from_u8(slice: &[u8]) -> Result<Metadata, DecodeError> {
        Ok(
            decode_from_slice(
                    slice,
                bincode::config::standard()
            )?.0
        )
    }
}
