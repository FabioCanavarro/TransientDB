//! # TransientDB 🦀
//!
//! An intelligent, persistent, and concurrent key-value store for Rust,
//! designed to manage data with a lifecycle.
//!
//! **TransientDB** is an opinionated database engine built on the robust foundation of `sled`.
//! It's designed specifically for workloads where the relevance of data changes over time,
//! such as caching, session management, and real-time analytics.
//!
//! It provides a high-level, ergonomic API by treating data's **access frequency**
//! and **age** as first-class citizens.

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
/// This struct holds the connection to the `sled` database and provides
/// safe, high-level access to the various data trees. It manages a background
/// thread for handling TTL (Time-To-Live) expirations automatically.
///
/// When this struct is dropped, it will signal the background thread to shut down
/// and wait for it to finish gracefully.
///
/// The main reason to why it stores 2 sled::Tree instead of a single sled::Db,
/// stems from the fact that almost all of the functions trees needs to be open which wastes some perfomance
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

/// This is a struct which contains all the key's additional information.
///
/// This struct contains the Serialize and Deserialize traits from serde as it is constantly
/// serialize into a &[u8] before being inserted into a tree and Deserialize when getting it from
/// the tree, for the reason that sled::tree only works with IVecs, where one of the supported
/// From<T> -> IVec  types is the &[u8]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// this is one of the key data that it holds which contain the amount of time the key has been
    /// get
    pub freq: u64,
    /// This is the created_at param which holds timestamp at which the key is created at, 
    /// 
    /// The timestamp is stored by getting the time since UNIX EPOCH as seconds
    /// so that it will be easier to just compare the time it is created to the current time,
    /// instead of having many extra functions
    pub created_at: u64,
    /// This is the ttl param which holds the time it has left to live, before it expires and gets
    /// deleted
    ///
    /// The ttl is stored in an Option<> to signify that the use of a ttl param is not a must which
    /// gives more flexibility to the user
    pub ttl: Option<u64>,
}
