use std::path::Path;
use std::time::Duration;
use epoch_db::DB;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new(Path::new("./databasetest"))?;
    db.backup_to(Path::new("./backup")).unwrap();

   Ok(())
}
