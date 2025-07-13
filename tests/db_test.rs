use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tempfile::tempdir;
use transient_db::DB;

#[test]
fn test_set() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice", None).unwrap();

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());
}

#[test]
fn test_rm() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice", None).unwrap();

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());

    db.remove("user:1").unwrap();

    let _ = db.get("user:1").unwrap().is_none();
}

#[test]
fn test_get_metadata() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice", None).unwrap();

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());

    db.increment_frequency("user:1").unwrap();

    let meta = db.get_metadata("user:1").unwrap().unwrap();

    assert_eq!(meta.freq, 1);

    sleep(Duration::new(1, 100));

    assert!(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            > meta.created_at
    )
}
