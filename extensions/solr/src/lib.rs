use polymath_crawler::extractor::meta::Meta;
use polymath_crawler::{Crawler, Event};

#[derive(Debug)]
struct Solr(Crawler);

impl Event for Solr {
    fn before_request(&self, _url: &str) -> Result<(), polymath_error::Error> {
        Ok(())
    }

    fn after_request(
        &self,
        _title: &str,
        _meta: Vec<Meta>,
        _html: &str,
    ) -> Result<(), polymath_error::Error> {
        Ok(())
    }
}
