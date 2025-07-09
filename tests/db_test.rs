use tempfile::tempdir;
use transient_db::DB;

#[test]
fn test_set() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice").unwrap();

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());
}

#[test]
fn test_rm() {
    let temp_dir = tempdir().unwrap();

    let db = DB::new(&temp_dir.path()).unwrap();

    db.set("user:1", "Alice").unwrap();

    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());

    db.remove("user:1").unwrap();

    let _ = db.get("user:1").unwrap().is_none();
}


