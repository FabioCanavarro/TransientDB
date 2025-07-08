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
        let _ =data_tree.insert(key, val);
        let _ = freq_tree.insert(key, Metadata::new().to_u8().expect("Cant serialize to u8"));
        
        todo!()
    }
}
