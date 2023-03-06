use std::collections::HashMap;
use aws_sdk_dynamodb::*;

pub async fn upload(map: &HashMap<String, HashMap<String, String>>) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await?;
    let client = Client::new(&config);
}