use std::path::Path;
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

    pub fn get(&self, key: String) -> Result<(), sled::Error> {
        let db = &self.db;
        let data_tree = db.open_tree("data_tree")?;
        let freq_tree = db.open_tree("freq_tree")?;
        // FIX: Proper error handling to take in Result<Option<>> rather than just Result<>
        let key = data_tree.get(key)?.expect("key is not found");
        let metadata = freq_tree.get(key)?.expect("freq is not found");
    }
}
