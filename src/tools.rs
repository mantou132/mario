use std::fs;
use std::io::Result;

use serde_json::Value as JsonValue;

pub fn deploy(root: &str, name: &JsonValue) -> Result<()> {
    fs::create_dir_all(root)?;

    let entrys = fs::read_dir(root)?;
    for entry in entrys {
        let entry: fs::DirEntry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() && file_name.to_str() == name.as_str() {
            // git pull
            println!("metadata: {:?}", metadata);
        } else {
            // git clone
        }
    }

    Ok(())
}