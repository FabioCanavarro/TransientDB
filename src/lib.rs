use db::errors::TransientError;
use serde::{Deserialize, Serialize};
use sled::Tree;
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread::JoinHandle,
};

pub mod db;
pub mod metadata;

/// This is the main struct which represents the database.
///
/// It stores 2 sled::Tree, instead of a single sled::Db,
/// Because in almost all of the functions trees needs to be open which wastes some perfomance
/// passing trees from the struct deletes the need of constant opening of the trees which increase
/// performance
#[derive(Debug)]
pub struct DB {
    /// the tree which stores the key and value
    data_tree: Arc<Tree>,
    /// the tree which stores the key and the metadata
    meta_tree: Arc<Tree>,
    /// the tree which stores the ttl timestamp and the key
    ttl_tree: Arc<Tree>,
    /// the thread that is used to iterate checks on the key with the earliest ttl timestamp
    ttl_thread: Option<JoinHandle<Result<(), TransientError>>>,
    /// the shutdown bool which tells the tt_thread that the DB is dropped so that the ttl_thread
    /// can join succesfully
    shutdown: Arc<AtomicBool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub freq: u64,
    // NOTE: second since the Unix epoch
    pub created_at: u64,
    pub ttl: Option<u64>,
}
