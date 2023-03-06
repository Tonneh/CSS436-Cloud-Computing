
pub async fn get_names(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;

    match response.status() {
        // 2xx status codes indicate success
        code if code.is_success() => Ok(response.text().await?),

        // 4xx and 5xx status codes indicate errors
        _ => Ok("Error".to_string()),
    }
}
