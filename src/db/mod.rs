pub mod errors;

use errors::TransientError;
use sled::{transaction::{ConflictableTransactionError, TransactionError, Transactional}, Config};
use std::{error::Error, path::Path, str::from_utf8, sync::{atomic::AtomicBool, Arc}, thread::{self, JoinHandle}, time::{Duration, SystemTime, UNIX_EPOCH}};

use crate::{DB, metadata::Metadata};

impl DB {
    pub fn new(path: &Path) -> Result<DB, sled::Error> {
        let db = Config::new()
            .path(path)
            .cache_capacity(512 * 1024 * 1024)
            .open()?;

        let data_tree = Arc::new(db.open_tree("data_tree")?);
        let meta_tree = Arc::new(db.open_tree("freq_tree")?);
        let ttl_tree = Arc::new(db.open_tree("ttl_tree")?);
        let ttl_tree_clone = Arc::clone(&ttl_tree);
        let meta_tree_clone = Arc::clone(&meta_tree);
        let data_tree_clone = Arc::clone(&data_tree);
        let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        let thread: JoinHandle<Result<(), TransientError>> = thread::spawn(
            move || {
                loop {
                    thread::sleep(Duration::new(0, 100000000));
                    if shutdown_clone.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    let keys = ttl_tree_clone.iter();
                    for i in keys {
                        let full_key = i.map_err(|e| TransientError::SledError { error: e })?;
                        // NOTE: The reason time is 14 u8s long is because it is being stored like
                        // this ([time,key], key) not ((time,key), key)
                        let key_byte = full_key.1;
                        let time_byte: [u8; 8] = (&full_key.0[..8]).try_into().map_err(|_| TransientError::ParsingToByteError)?;
                        let time = u64::from_be_bytes(time_byte);
                        let curr_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Cant get SystemTime").as_secs();

                        if curr_time >= time {
                            let l: Result<(), TransactionError<()>> = (&*data_tree_clone, &*meta_tree_clone, &*ttl_tree_clone).transaction(
                                        |(data, freq, ttl_tree_clone)| 
                                        {
                                            let byte = &key_byte;
                                            data.remove(byte)?;
                                            freq.remove(byte)?;
                                            
                                            let _ = ttl_tree_clone.remove([&time_byte, &byte[..]].concat());

                                            Ok(())
                                        }
                                    );
                                    l.map_err(|_| TransientError::SledTransactionError)?;
                        }
                        // NOTE: Use transaction to delete tomorrow, imma head out
                    }
                };
                Ok(())
                
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
            Some(t) => {
                let systime = SystemTime::now().duration_since(UNIX_EPOCH).expect("Cant get SystemTime");
                Some((t + systime).as_secs())
            },
            None => None
        };


        let l: Result<(), TransactionError<()>> = (&**data_tree, &**freq_tree, &**ttl_tree).transaction(
            |(data, freq, ttl_tree)| {

                match freq.get(byte)? {
                    Some(m) => {
                        let mut meta = Metadata::from_u8(&m.to_vec()).map_err(|_| ConflictableTransactionError::Abort(()))?;
                        if let Some(t) = meta.ttl {
                            let _ = ttl_tree.remove([&t.to_be_bytes()[..], &byte[..]].concat());
                        }
                        meta.ttl = ttl_sec; 
                        freq.insert(
                            byte,
                            meta.to_u8().expect("Cant serialize to u8"),
                        )?;

                    },
                    None => {
                        freq.insert(
                            byte,
                            Metadata::new(ttl_sec).to_u8().expect("Cant serialize to u8"),
                        )?;

                    }
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
        let _ = l.map_err(|_| TransientError::SledTransactionError)?;

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
                .ok_or(TransientError::IncretmentError)?;
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
        let ttl_tree = &self.ttl_tree;
        let byte = &key.as_bytes();
        let l: Result<(), TransactionError<()>> = (&**data_tree, &**freq_tree, &**ttl_tree).transaction(
            |(data, freq, ttl_tree)| 
            {
                data.remove(*byte)?;
                let meta = freq.get(byte)?.ok_or(ConflictableTransactionError::Abort(()))?;
                let time = Metadata::from_u8(&meta.to_vec()).map_err(|_| ConflictableTransactionError::Abort(()))?.ttl;
                freq.remove(*byte)?;
                
                match time {
                    Some(t) => 
                    {
                        let _ = ttl_tree.remove([&t.to_be_bytes()[..], &byte[..]].concat());
                    },
                    None => ()
                    
                }

                Ok(())
            }
        );
        l.map_err(|_| TransientError::SledTransactionError)?;
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

        let _ = self.ttl_thread.take().expect("Fail to take ownership of ttl_thread").join().expect("Joining failed");
    }
}























