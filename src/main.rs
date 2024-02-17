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

    let mime_type = "image/png";
    let bucket_name = "rust-test";
    let folder_name = "avatar";
    let file_name = "image.png";
    let file_path = format!("{}/{}", folder_name, file_name);
    let _object = cloud_storage::Client::new()
        .object()
        .create(&bucket_name, buf, &file_path, mime_type)
        .await?;

    Ok(())
}