# Robots.txt

This extension allows web crawlers to access and respect instructions from a website's `robots.txt` file before starting the crawl.

A `robots.txt` file is a text file on a website that tells search engine crawlers (like bots) which parts of the site they can access.
The extension supports two main directives from `robots.txt`:

* `Crawl-delay`: specify and set the number of seconds the robot must wait between each successive request.
* `Disallow`: prevents the crawler from accessing specific URLs or directories on the website.

The extension also checks for meta robots tags in web pages, which provide additional instructions for crawling alongside robots.txt.  For more information on meta robots, see [More information](https://robots-txt.com/meta-robots/).

The extension can also be used to get sitemaps from robots.txt.

## Example

```rust
use polymath_crawler::Crawler;
use robots::Extension;

#[tokio::main]
async fn main() {
    // Create custom crawler.
    let crawler = Crawler::new()
        .with_robots_txt(true)
        .with_name("Gravitaliabot");

    // Start crawling websites.
    // It will firsly check https://example.com/robots.txt before
    // crawling site.
    crawler.fetch(vec!["https://example.com/"]).await;
}
```