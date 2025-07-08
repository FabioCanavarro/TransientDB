use std::{error::Error, path::Path, str::from_utf8};
use sled::{Config, Tree};

use crate::metadata::Metadata;

#[derive(Debug)]
struct DB {
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
            DB { data_tree, meta_tree }
        )
    }
    pub fn set(&self, key: &str, val: &str) -> Result<(), sled::Error>{
        let data_tree = &self.data_tree;
        let _ =data_tree.insert(key.as_bytes(), val.as_bytes())?;

        Ok(())
    }

    pub fn set_overwrite_metadata(&self, key: &str, val: &str) -> Result<(), sled::Error>{
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let _ =data_tree.insert(key.as_bytes(), val.as_bytes())?;
        let _ = freq_tree.insert(key.as_bytes(), Metadata::new().to_u8().expect("Cant serialize to u8"))?;
        
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let byte = &key.as_bytes();
        let val = data_tree.get(byte)?.expect("Key returns none");

        Ok(from_utf8(&val.to_vec())?.to_string())
    }

    pub fn increment_frequency(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();

        let metadata = freq_tree.get(byte)?.expect("freq is not found");
        let mut meta = Metadata::from_u8(&metadata.to_vec()[..]).expect("Cant deserialize from freq_tree to Metadata");
        meta.freq_incretement();
        let _ = freq_tree.insert(&key, meta.to_u8()?)?;

        Ok(())

    }

    pub fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let data_tree = &self.data_tree;
        let freq_tree = &self.meta_tree;
        let byte = &key.as_bytes();
        let _ = data_tree.remove(byte)?;
        let _ =freq_tree.remove(byte)?;
        Ok(())
        

    }
}
