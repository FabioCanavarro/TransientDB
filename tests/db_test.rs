use transient_db::DB;
use std::env::temp_dir;

#[test]
fn test_set() {

    let temp_dir =temp_dir();
    println!("here");
    let db = DB::new(&temp_dir).unwrap();
    println!("here");
    db.set("user:1", "Alice").unwrap();
    println!("here");
 
    assert_eq!("Alice", db.get("user:1").unwrap().unwrap());
}
