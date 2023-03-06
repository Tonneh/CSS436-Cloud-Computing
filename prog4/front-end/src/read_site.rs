use reqwasm::http::Request;

/* Reads text from site and returns as string */
pub async fn read_text(url: &str) -> String {
    let response = match Request::get(&url).send().await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error fetching {}: {}", url, e);
            return String::new();
        }
    };
    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading response text: {}", e);
            return String::new();
        }
    };
    text
}
