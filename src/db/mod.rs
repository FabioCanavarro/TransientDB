use std::{error::Error, path::Path, str::from_utf8};
use sled::{Config, Db};

use crate::metadata::Metadata;

#[derive(Debug)]
struct DB {
    db: Db
}

impl DB {
    pub fn new(path: &Path) -> Result<Db, sled::Error> {
        let db = Config::new()
            .path(path)
            .cache_capacity(512 * 1024 * 1024) 
            .open()?;
        let _ = db.open_tree("data_tree")?;
        let _ = db.open_tree("freq_tree")?;
        Ok(db)
    }

    pub fn set(&self, key: &str, val: &str) -> Result<(), sled::Error>{
        let db = &self.db;
        let data_tree = db.open_tree("data_tree")?;
        let freq_tree = db.open_tree("freq_tree")?;
        let _ =data_tree.insert(key.as_bytes(), val.as_bytes())?;
        let _ = freq_tree.insert(key.as_bytes(), Metadata::new().to_u8().expect("Cant serialize to u8"))?;
        
        todo!()
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let db = &self.db;
        let byte = &key.as_bytes();
        let data_tree = db.open_tree("data_tree")?;
        let freq_tree = db.open_tree("freq_tree")?;
        // FIX: Proper error handling to take in Result<Option<>> rather than just Result<>
        
        let val = data_tree.get(byte)?.expect("val is not found");

        let metadata = freq_tree.get(byte)?.expect("freq is not found");
        let mut meta = Metadata::from_u8(&metadata.to_vec()[..]).expect("Cant deserialize from freq_tree to Metadata");
        meta.freq_incretement();
        let _ = freq_tree.insert(&val, meta.to_u8()?)?;
        Ok(from_utf8(&val.to_vec())?.to_string())
    }

    pub fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let db = &self.db;
        let byte = &key.as_bytes();
        let data_tree = db.open_tree("data_tree")?;
        let freq_tree = db.open_tree("freq_tree")?;
        let _ = data_tree.remove(byte)?;
        let _ =freq_tree.remove(byte)?;
        Ok(())
        

    }
}
