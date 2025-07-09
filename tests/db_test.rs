use transient_db::db::DB;
use std::env::temp_dir;

fn test_set() -> Result<(), Box<dyn std::error::Error>> {

    let temp_dir =temp_dir();
    let db = DB::new(temp_dir)?;

    db.set("user:1", "Alice")?;
    
    if let Some(value) = db.get("user:1")? {
        println!("Found value: {}", value); // "Found value: Alice"
    }

    db.increment_frequency("user:1")?;

    db.remove("user:1")?;

    Ok(())
}
