use sled::{Db, Tree};

pub mod db;
pub mod metadata;

#[derive(Debug)]
pub struct DB {
    db: Db,
    data_tree: Tree,
    meta_tree: Tree
}
