use std::time::{SystemTime, UNIX_EPOCH};
use crawl::crawler_client::CrawlerClient;
use crawl::CrawlRequest;


pub mod crawl {
    tonic::include_proto!("crawl");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CrawlerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CrawlRequest {
        url: "https://www.gravitalia.studio/".into()
    });

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let response = client.crawl_site(request).await?;
    println!("Response with {:?} in {}ms", response.into_inner().message, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()-start);

    Ok(())
}