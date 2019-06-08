use std::fs;
use std::io::Result;
use std::process::Command;

use crate::global::CONFIG;

pub fn deploy(name: &str, clone_url: &str) -> Result<()> {
    let root = CONFIG
        .get()
        .expect("Config is not initialized")
        .root
        .clone();
    fs::create_dir_all(&root)?;

    let entrys = fs::read_dir(&root)?;
    let mut dir = None;
    for entry in entrys {
        let entry: fs::DirEntry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() && file_name.to_str() == Some(name) {
            dir = Some(fs::canonicalize(format!("./{}", name))?);
            break;
        }
    }
    if let Some(path) = dir {
        Command::new("git")
            .arg("pull")
            .current_dir(path)
            .status()
            .expect("failed to execute `git pull` process");
    } else {
        Command::new("git")
            .arg("clone")
            .arg(clone_url)
            .arg(name)
            .status()
            .expect("failed to execute `git clone` process");
    }

    execute_after_script();
    Ok(())
}

fn execute_after_script() {
    let after_script = CONFIG
        .get()
        .expect("Config is not initialized")
        .after_script
        .clone();
    let split: Vec<&str> = after_script[..].split(' ').collect();
    let mut cmd = Command::new(split[0]);
    for arg in split.iter().skip(1) {
        cmd.arg(arg);
    }
    cmd.status()
        .expect("failed to execute after_script process");
}