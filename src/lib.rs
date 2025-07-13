use db::errors::TransientError;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub freq: u64,
    // NOTE: second since the Unix epoch
    pub created_at: u64,
    pub ttl: Option<u64>,
}
