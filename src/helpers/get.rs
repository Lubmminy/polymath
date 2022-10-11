use hyper::Client;

pub async fn get(_url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    Ok(())
}