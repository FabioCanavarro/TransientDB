pub mod errors;

use std::{error::Error, path::Path, str::from_utf8};
use errors::TransientError;
use sled::{transaction::TransactionError, Config, Db, Tree};

use crate::metadata::Metadata;

#[derive(Debug)]
struct DB {
    db: Db,
    data_tree: Tree,
    meta_tree: Tree
}

impl DB {
    pub fn new(path: &Path) -> Result<DB, sled::Error> {
        let db = Config::new()
            .path(path)
            .cache_capacity(512 * 1024 * 1024) 
            .open()?;
        let data_tree = db.open_tree("data_tree")?;
        let meta_tree = db.open_tree("freq_tree")?;
        Ok(
            DB { db, data_tree, meta_tree }
        )
    }
    pub fn set(&self, key: &str, val: &str) -> Result<(), Box<dyn Error>>{
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let byte = key.as_bytes();
        let l: Result<(), TransactionError> = self.db.transaction(
            |_| {
                if !freq_tree.contains_key(byte)? {
                    freq_tree.insert(key.as_bytes(), Metadata::new().to_u8().expect("Cant serialize to u8"))?;
                }
                data_tree.insert(key.as_bytes(), val.as_bytes())?;

                Ok(())
            }
        );
        l?;

        Ok(())
    }

    pub fn set_overwrite_metadata(&self, key: &str, val: &str) -> Result<(), Box<dyn Error>>{
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let l: Result<(),TransactionError> = self.db.transaction(
            |_| {
                data_tree.insert(key.as_bytes(), val.as_bytes())?;
                freq_tree.insert(key.as_bytes(), Metadata::new().to_u8().expect("Cant serialize to u8"))?;
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
            None => Ok(None)
            
        }

    }

    pub fn increment_frequency(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();

        loop {
            let metadata = freq_tree.get(byte)?.unwrap_or(Err(TransientError::IncretmentFailure)?);
            let meta = Metadata::from_u8(&metadata.to_vec()[..])?;
            let s = freq_tree.compare_and_swap(byte, Some(metadata), Some(meta.freq_incretement().to_u8()?));
            match s {
                Ok(_) => break,
                Err(_) => ()
            }
        }

        Ok(())

    }

    pub fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();
        let l: Result<(), TransactionError> = self.db.transaction(
            |_| {
                    data_tree.remove(byte)?;
                    freq_tree.remove(byte)?;

                    Ok(())
            }
        );
        l?;
        Ok(())
        

    }
}
