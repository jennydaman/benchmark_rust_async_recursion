use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let size = du(path);
    println!("{size}")
}

fn du(path: &Path) -> u64 {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.is_symlink() {
                metadata.len()
            } else if metadata.is_dir() {
                match fs::read_dir(path) {
                    Ok(rd) => {
                        metadata.len()
                            + rd.map(|r| r.map(|entry| du(&entry.path())).unwrap_or(0))
                                .sum::<u64>()
                    }
                    Err(_e) => 0,
                }
            } else {
                metadata.len()
            }
        }
        Err(_e) => 0,
    }
}
