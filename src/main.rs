use std::fs::File;
use std::io::Read;
use cloud_storage::Error;

#[tokio::main]
async fn main() {
    match store().await {
        Ok(_) => println!("File stored successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

async fn store() -> Result<(), Error> {
    let mut file = File::open("src/images/img.png")?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    const MIME_TYPE: &str = "image/png";
    const BUCKET_NAME: &str = "rust-test";
    const DIR_NAME: &str = "avatar";
    const FILE_NAME: &str = "image.png";
    let file_path = format!("{}/{}", DIR_NAME, FILE_NAME);
    let _object = cloud_storage::Client::new()
        .object()
        .create(&BUCKET_NAME, buf, &file_path, MIME_TYPE)
        .await?;

    Ok(())
}