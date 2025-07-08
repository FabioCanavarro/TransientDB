use std::path::Path;
use sled::{Config, Db};

#[derive(Debug)]
struct DB {
    db: Db
}

impl DB {
    fn new(path: &Path) -> Result<Db, sled::Error> {
        let db = Config::new()
            .path(path)
            .cache_capacity(512 * 1024 * 1024) 
            .open()?;
        let _ = db.open_tree("data_tree")?;
        let _ = db.open_tree("freq_tree")?;
        Ok(db)
    }
}
