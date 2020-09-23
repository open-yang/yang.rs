use std::fs::{self, read_to_string};
use std::io;
use std::path::Path;

pub fn parse_yang(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                parse_yang(&path)?;
            } else if let Some(extension) = path.extension() {
                if extension.eq("yang") {
                    parse(&path)?;
                }
            }
        }
    }
    Ok(())
}

fn parse(file: &Path) -> io::Result<()> {
    if file.extension().unwrap().eq("yang") {
        println!("-------------------------------------");
        println!("{}", file.to_str().unwrap());
        println!("{}", read_to_string(file)?);
        println!("-------------------------------------");
    }
    Ok(())
}