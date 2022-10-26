use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body, Method, Request, Version};
use async_recursion::async_recursion;
use robotstxt::DefaultMatcher;
use std::{thread, time};
use std::str;
use url::Url;
use hyper::body;

pub async fn init(homepage: String, client: &Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    if regex::Regex::new(r"twitter.com|cloudfront.net|youtube.com|instagram.com|linkedin.com|github.com|wikipedia.org").unwrap().is_match(&homepage) {
        return Ok("blacklisted")
    }
    let robots = get(&format!("{}/robots.txt", homepage), client, "User-Agent: *\nAllow: /\n".to_string()).await?;

    let res = get(&homepage, client, robots).await;
    match res {
        Ok(v) => {
            if v == "not allowed".to_string() {
                Ok("not allowed")
            } else {
                Ok("crawling")
            }
        },
        Err(e) => {
            println!("{:?}", e);
            Ok("not allowed")
        },
    }
}

#[async_recursion]
pub async fn get(url: &String, client: &Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>, robots: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut matcher = DefaultMatcher::default();
    if !matcher.one_agent_allowed_by_robots(&robots, "Gravitaliabot", &url) {
        return Ok("not allowed".to_string())
    }
    let crawl_delay = if regex::Regex::new(r"[C|c]rawl-[D|d]elay:\s?([0-9]{1,4})").unwrap().captures_iter(&robots).last().is_some() {
        regex::Regex::new(r"[C|c]rawl-delay:\s?([0-9]{1,4})").unwrap().captures_iter(&robots).last().unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap()
    } else {
        0
    };

    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("User-Agent", "Gravitaliabot/0.1")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("Accept-Language", "en-US;q=0.9,en;q=0.8")
        .version(Version::HTTP_11)
        .body(Body::default())?;
    let res = client.request(req).await?;

    if !regex::Regex::new(r"[2|3][0|2][0-9]").unwrap().is_match(res.status().as_str()) {
        return Ok("status".to_string())
    }

    let body_res = String::from_utf8(body::to_bytes(res.into_body()).await?.to_vec()).expect("response was not valid utf-8");
    for site in regex::Regex::new("https:/{2}[a-z|0-9|-|_]*[.]?[a-z|0-9|-|_]{1,256}[.][a-z]{2,63}(/[A-z|0-9|_|~|/|-]*)?[.]?([a-z]{1,15})?").unwrap().captures_iter(&body_res) {
        let ext = if site.get(2).is_some() {
            site.get(2).unwrap().as_str()
        } else { "ok" };

        if ![ "webp", "png", "xml", "toml", "jpg", "tiff", "gif", "txt", "avif" ].contains(&ext) {
            let site_url = site.get(0).unwrap().as_str();
            if Url::parse(&url).unwrap().host_str().unwrap() == Url::parse(&site_url).unwrap().host_str().unwrap() {
                thread::sleep(time::Duration::from_secs(crawl_delay)); // Wait the Crawl-Delay
                if Url::parse(&site_url).unwrap().path() != Url::parse(&url).unwrap().path() {
                    let _ = get(&site_url.to_string(), client, robots.clone()).await;
                }
            } else {
                let _ = super::nats::publish(format!("https://{}", Url::parse(&site_url).unwrap().host_str().unwrap().to_string()));
            }
        }
    }

    Ok(body_res)
}