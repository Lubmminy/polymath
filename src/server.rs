use tonic::{transport::Server, Request, Response, Status};

use crawl::crawler_server::{Crawler, CrawlerServer};
use crawl::{CrawlReply, CrawlRequest};

use hyper::{Client, client::{HttpConnector, connect::dns::GaiResolver}, Body};
use hyper_tls::HttpsConnector;

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
        println!("Got a request for crawling {:?}", url);

        println!("{:?}", helpers::get::get(&url, self.client.clone()).await);

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

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CrawlerServer::new(PolyMath { client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()) }))
        .serve(addr)
        .await?;

    Ok(())
}