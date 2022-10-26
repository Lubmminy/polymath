use tonic::{transport::Server, Request, Response, Status};

use crawl::crawler_server::{Crawler, CrawlerServer};
use crawl::{CrawlReply, CrawlRequest};

use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body};
use hyper_tls::HttpsConnector;
use url::Url;

mod helpers;
pub mod crawl {
    tonic::include_proto!("crawl");
}

pub struct PolyMath {
    client: Client<hyper_tls::HttpsConnector<HttpConnector<GaiResolver>>, Body>
}

#[tonic::async_trait]
impl Crawler for PolyMath {
    async fn crawl_site(
        &self,
        request: Request<CrawlRequest>,
    ) -> Result<Response<CrawlReply>, Status> {
        let url = request.into_inner().url;

        let site_url = Url::parse(&url).unwrap();
        if site_url.scheme() != "https" {
            let err_res = CrawlReply {
                message: "Please use HTTPS scheme".to_string(),
                error: true
            };
            println!("Can't crawl it: invalid scheme");
            return Ok(Response::new(err_res))
        }

        let _ = helpers::get::init(format!("https://{}", site_url.host_str().unwrap()), &self.client).await;

        let reply = CrawlReply {
            message: format!("Crawling {}...", url),
            error: false
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let mut already_crawled: Vec<String> = [].to_vec();

    tokio::spawn(async move {
        for msg in helpers::nats::init().await.unwrap().messages() {
            if already_crawled.contains(&std::str::from_utf8(&msg.data).unwrap().to_string()) {
                already_crawled.push(std::str::from_utf8(&msg.data).unwrap().to_string());
                tokio::spawn(async move {
                    let _ = helpers::get::init(std::str::from_utf8(&msg.data).unwrap().to_string(), &Client::builder().build::<_, hyper::Body>(HttpsConnector::new())).await;
                });
            }
        }
    });

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(CrawlerServer::new(PolyMath { client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()) }))
        .serve(addr)
        .await?;

    Ok(())
}