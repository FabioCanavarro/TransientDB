use std::path::Path;
use std::time::Duration;
use epoch_db::DB;


// NOTE: Testing to see what sled outputs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new(Path::new("./my_database"))?;

    for i in 0..100 {
        db.set("{i}", "{i}", Some(Duration::from_secs(10)))?;

        if let Some(value) = db.get("{i}")? {
            println!("Found value: {}", value); // "Found value: Alice"
        }

        db.increment_frequency("{i}").unwrap();

    }
   Ok(())
}
