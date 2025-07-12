pub mod errors;

use errors::TransientError;
use sled::{Config, transaction::TransactionError, transaction::Transactional};
use std::{error::Error, path::Path, str::from_utf8, sync::{atomic::AtomicBool, Arc, Mutex}, thread, time::Duration};

use crate::{DB, metadata::Metadata};

impl DB {
    pub fn new(path: &Path) -> Result<DB, sled::Error> {
        let db = Config::new()
            .path(path)
            .cache_capacity(512 * 1024 * 1024)
            .open()?;

        let data_tree = db.open_tree("data_tree")?;
        let meta_tree = db.open_tree("freq_tree")?;
        let ttl_tree = db.open_tree("ttl_tree")?;
        let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        let thread = thread::spawn(
            move || {
                loop {
                    thread::sleep(Duration::new(0, 100000000));
                    if shutdown_clone.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    let keys = db.open_tree("ttl_tree").expect("cant open tree").iter();
                    for i in keys {
                        let full_key = i.unwrap();
                        let time = full_key.0;
                        let key = full_key.1;
                        let byte: [u8; 8] = time[..].try_into().expect("Cant convert from key to [u8; 8]");
                        println!("{} : {}", from_utf8(&key[..]).expect("Cant convert from [u8] to utf8").to_string(),u64::from_be_bytes(byte));
                    }
                }
                
            }
        );
        Ok(DB {
            data_tree,
            meta_tree,
            ttl_tree,
            ttl_thread: Some(thread),
            shutdown
        })
    }
    pub fn set(&self, key: &str, val: &str, ttl: Option<Duration>) -> Result<(), Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let ttl_tree = &self.ttl_tree;
        let byte = key.as_bytes();
        let ttl_sec = match ttl {
            Some(t) => Some(t.as_secs()),
            None => None
        };

        let l: Result<(), TransactionError> = (data_tree, freq_tree, ttl_tree).transaction(
            |(data, freq, ttl_tree)| {
                if freq.get(byte)?.is_none() {
                    freq.insert(
                        byte,
                        Metadata::new(ttl_sec).to_u8().expect("Cant serialize to u8"),
                    )?;
                    assert!(freq.get(byte)?.is_some());

                }

                data.insert(byte, val.as_bytes())?;

                match ttl_sec {
                    Some(d) => {
                        ttl_tree.insert(
                            [&d.to_be_bytes()[..], &byte[..]].concat(),
                            byte
                        )?;
                    },
                    None => ()
                };

                Ok(())
            }
        );
        l?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let byte = key.as_bytes();
        let val = data_tree.get(byte)?;
        match val {
            Some(val) => Ok(Some(from_utf8(&val.to_vec())?.to_string())),
            None => Ok(None),
        }
    }

    pub fn increment_frequency(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();

        loop {
            let metadata = freq_tree
                .get(byte)?
                .ok_or(TransientError::IncretmentFailure)?;
            let meta = Metadata::from_u8(&metadata.to_vec())?;
            let s = freq_tree.compare_and_swap(
                byte,
                Some(metadata),
                Some(meta.freq_incretement().to_u8()?),
            );
            match s {
                Ok(_) => break,
                Err(_) => (),
            }
        }

        Ok(())
    }

    pub fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();
        let l: Result<(), TransactionError> = (data_tree, freq_tree).transaction(|(data, freq)| {
            data.remove(*byte)?;
            freq.remove(*byte)?;

            Ok(())
        });
        l?;
        Ok(())
    }

    pub fn get_metadata(&self, key: &str) -> Result<Option<Metadata>, Box<dyn Error>> {
        let freq_tree = &self.meta_tree;
        let byte = key.as_bytes();
        let meta = freq_tree.get(byte)?;
        match meta {
            Some(val) => Ok(Some(Metadata::from_u8(&val.to_vec())?)),
            None => Ok(None),
        }
    }
}

impl Drop for DB {
    fn drop(&mut self) {
       self.shutdown
            .store(true, std::sync::atomic::Ordering::SeqCst);

        self.ttl_thread.take().expect("Fail to take ownership of ttl_thread").join().expect("Joining failed");
    }
}























