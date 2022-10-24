use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body, Method, Request, Version};
use async_recursion::async_recursion;
use hyper::body::HttpBody;
use robotstxt::DefaultMatcher;
use std::str;
use url::Url;

pub async fn init(homepage: String, client: Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    println!("Start crawling {}...", homepage);
    let robots = get(&format!("{}/robots.txt", homepage), client.clone(), "User-Agent: *\nAllow: /\n".to_string()).await;

    let res = get(&homepage, client, robots.unwrap()).await.unwrap();
    println!("{}", res);
    if res == "not allowed".to_string() {
        Ok("not allowed")
    } else {
        Ok("crawling")
    }
}

#[async_recursion]
pub async fn get(url: &String, client: Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>, robots: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut matcher = DefaultMatcher::default();
    if !matcher.one_agent_allowed_by_robots(&robots, "Gravitaliabot", &url) {
        return Ok("not allowed".to_string())
    }

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
        return Ok("status".to_string())
    }
    
    while let Some(chunk) = res.body_mut().data().await {
        //let test = &chunk;
        for site in regex::bytes::Regex::new("https:/{2}([a-z|0-9|-|_]*.){0,10}[.]([a-z]{2,63})").unwrap().captures(&chunk?) {
            println!("Find an URL: {}", str::from_utf8(site.get(0).unwrap().as_bytes())?.to_string());
            if ![ "webp", "png", "xml", "toml", "jpg", "tiff", "gif", "txt", "avif" ].contains(&str::from_utf8(site.get(2).unwrap().as_bytes()).unwrap()) {
                let site_url = str::from_utf8(site.get(0).unwrap().as_bytes())?.to_string();
                if Url::parse(&url).unwrap().host_str().unwrap() == Url::parse(&site_url).unwrap().host_str().unwrap() {
                    get(&site_url, client.clone(), robots.clone()).await;
                } else {
                    println!("here");
                    // add queue normally
                    init(Url::parse(&site_url).unwrap().host_str().unwrap().to_string(), client.clone()).await;
                }
                println!("{:?}", str::from_utf8(site.get(0).unwrap().as_bytes())?.to_string());
            } else {
                // download and analyze images and files here
            }
        }
        //return Ok(str::from_utf8(&chunk?).unwrap().to_string())
        return Ok("crawled".to_string())
    }

    Ok("error".to_string())
}