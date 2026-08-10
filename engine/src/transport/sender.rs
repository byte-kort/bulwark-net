use reqwest::Client;

pub async fn send_json(json: String) -> Result<(), Box< dyn std::error::Error >> {
    let client = Client::new();
    let response = client
        .post("https://bulwark-network.net/api/events")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;

    response.error_for_status()?;

    Ok(())
}
