use db::errors::TransientError;
use sled::Tree;
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread::JoinHandle,
};

pub mod db;
pub mod metadata;

#[derive(Debug)]
pub struct DB {
    data_tree: Arc<Tree>,
    meta_tree: Arc<Tree>,
    ttl_tree: Arc<Tree>,
    ttl_thread: Option<JoinHandle<Result<(), TransientError>>>,
    shutdown: Arc<AtomicBool>,
}
