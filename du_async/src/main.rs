use std::env;
use std::path::Path;
use async_recursion::async_recursion;
use futures::StreamExt;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let size = du(path).await;
    println!("{}", size)
}


#[async_recursion]
async fn du(path: &Path) -> u64 {
    match tokio::fs::symlink_metadata(path).await {
        Ok(metadata) => {
            if metadata.is_symlink() {
                metadata.len()
            } else if metadata.is_dir() {
                match tokio::fs::read_dir(path).await.map(tokio_stream::wrappers::ReadDirStream::new) {
                    Err(_e) => metadata.len(),
                    Ok(rds) => {
                        metadata.len() +
                        rds.map(|r| async move {
                            if let Ok(entry) = r {
                                du(&entry.path()).await
                            } else {
                                0
                            }
                        })
                            .buffer_unordered(1)
                            .fold(0, |a, x| async move {a + x}).await
                    }
                }

            } else {
                metadata.len()
            }
        },
        Err(_e) => 0
    }
}
