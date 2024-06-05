# Sitemap

This extension allows you to access and read the sitemaps provided by the site. Either by looking directly at `/sitemap.xml`, or via `/robots.txt`.

## Example

```rust
use polymath_crawler::Crawler;
use sitemap::Extension;

#[tokio::main]
async fn main() {
    // Create custom crawler.
    let crawler = Crawler::new()
        .with_sitemap(true);

    // Start crawling websites.
    // It will firsly check https://example.com/robots.txt before
    // crawling site.
    crawler.fetch(vec!["https://example.com/"]).await;
}
```