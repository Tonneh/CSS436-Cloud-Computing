use aws_sdk_s3::*;
use aws_sdk_s3::types::ByteStream;

use crate::http_call::get_site_data::get_names;

use std::path::Path;

pub async fn s3_upload(path_to_file: &str, s3_bucket: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let file = ByteStream::from_path(Path::new(path_to_file)).await;

    match file {
        Ok(f) => {
            client
                .put_object()
                .bucket(s3_bucket)
                .key(key)
                .body(f)
                .send()
                .await?
        },
        Err(e) => {
            panic!("Error uploading file: {:?}", e);
        }
    };
    Ok(())
}