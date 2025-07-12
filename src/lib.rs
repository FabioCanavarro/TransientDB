use std::{error::Error, sync::{atomic::AtomicBool, Arc}, thread::JoinHandle};
use sled::Tree;

pub mod db;
pub mod metadata;

#[derive(Debug)]
pub struct DB {
    data_tree: Tree,
    meta_tree: Tree,
    ttl_tree: Arc<Tree>,
    ttl_thread: Option<JoinHandle<Result<(), Box<dyn Error + Send>>>>,
    shutdown: Arc<AtomicBool>
}

