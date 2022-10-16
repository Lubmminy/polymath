use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body, Method, Request, Version};
use hyper::body::HttpBody;

pub async fn get(url: &String, client: Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>) -> Result<&str, Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("User-Agent", "Gravitaliabot/0.1")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("Accept-Language", "en-US;q=0.9,en;q=0.8")
        .version(Version::HTTP_11)
        .body(Body::default())?;

    let mut res = client.request(req).await?;


    if !regex::Regex::new(r"[2|3][0|2][0-9]").unwrap().is_match(res.status().as_str()) {
        return Ok("status")
    }

    while let Some(chunk) = res.body_mut().data().await{
        println!("{:?}", &chunk?);
    }

    println!("{}", regex::Regex::new(r"[2|3][0|2][0-9]").unwrap().is_match(res.status().as_str()));
    Ok("ok")
}