use std::path::Path;
use sled::Db;

#[derive(Debug)]
struct DB {
    db: Db
}

impl DB {
    fn new(path: &Path) -> Result<Db, sled::Error> {
        let db = sled::open(
            path
        )?;
        let _ = db.open_tree("data_tree")?;
        let _ = db.open_tree("freq_tree")?;
        Ok(db)
    }
}
