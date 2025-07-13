use std::{sync::{atomic::AtomicBool, Arc}, thread::JoinHandle};
use db::errors::TransientError;
use sled::Tree;

pub mod db;
pub mod metadata;

#[derive(Debug)]
pub struct DB {
    data_tree: Arc<Tree>,
    meta_tree: Arc<Tree>,
    ttl_tree: Arc<Tree>,
    ttl_thread: Option<JoinHandle<Result<(), TransientError>>>,
    shutdown: Arc<AtomicBool>
}

