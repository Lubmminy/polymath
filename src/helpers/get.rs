use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body, Method, Request};
use hyper::body::HttpBody;

pub async fn get(url: String, client: Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("User-Agent", "Gravitaliabot/0.1")
        .body(Body::from(r#"{"sorry":"we're crawling"}"#))?;

    let mut res = client.request(req).await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    while let Some(chunk) = res.body_mut().data().await{
        println!("{:?}", &chunk?);
    }
    Ok(())
}