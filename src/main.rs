use hyper::Client;
use hyper_tls::HttpsConnector;
mod helpers;

#[tokio::main]
async fn main() {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    println!("{:?}", helpers::get::get("test".to_string(), client).await);
}
