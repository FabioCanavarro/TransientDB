use std::{sync::{Arc, Mutex}, thread::JoinHandle};
use sled::Tree;

pub mod db;
pub mod metadata;

#[derive(Debug)]
pub struct DB {
    data_tree: Tree,
    meta_tree: Tree,
    ttl_tree: Tree,
    ttl_thread: Option<JoinHandle<()>>
}

