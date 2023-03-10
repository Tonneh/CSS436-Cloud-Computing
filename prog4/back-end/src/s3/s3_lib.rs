use aws_sdk_s3::model::ObjectCannedAcl;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::*;
use std::collections::HashMap;
use std::path::Path;

pub async fn s3_upload(
    path_to_file: &str,
    s3_bucket: &str,
    key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let file = ByteStream::from_path(Path::new(path_to_file)).await;

    match file {
        Ok(f) => {
            client
                .put_object()
                .set_acl(Option::from(ObjectCannedAcl::PublicRead))
                .set_metadata(Option::from(HashMap::from([(
                    "content-type".to_string(),
                    "text/plain".to_string(),
                )])))
                .bucket(s3_bucket)
                .key(key)
                .body(f)
                .send()
                .await?
        }
        Err(e) => {
            panic!("Error uploading file: {:?}", e);
        }
    };
    Ok(())
}
