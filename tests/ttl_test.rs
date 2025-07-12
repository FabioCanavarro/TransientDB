use std::{thread::sleep, time::{Duration}};

use tempfile::tempdir;
use transient_db::DB;

#[test]
fn test_ttl() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice", Some(Duration::new(10, 0))).unwrap();

    sleep(Duration::new(10, 0));

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());
}
