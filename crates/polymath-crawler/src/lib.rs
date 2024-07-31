#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! fetch and extract datas from website.

pub mod extractor;

#[macro_use]
extern crate lazy_static;

use extractor::meta::Meta;
use polymath_cache::lru::LRUCache;
use polymath_error::CrawlerError;
use regex_lite::Regex;
use std::{collections::HashMap, fmt::Debug, time::Duration};
use tracing::{debug, error};
use ureq::{Agent, Request};

const ALLOWED_EXT: [&str; 16] = [
    "pdf", // Adobe Portable Document Format
    "ppt", "pptx", // Microsoft PowerPoint
    "doc", "docx", // Microsoft Word
    "odp",  // OpenOffice
    "tex",  // Tex/LaTex
    "txt",  // Text
    "jpeg", "jpg", "png", "webp", "gif", // Images
    "mp4", "ogv", "mov", // Videos
];

/// [Crawler]-related events.
///
/// # Examples
/// ```rust
/// use polymath_crawler::{Crawler, Event};
/// use polymath_crawler::extractor::meta::Meta;
///
/// #[derive(Debug)]
/// struct Solr(Crawler);
///
/// impl Event for Solr {
///     fn before_request(&self, url: &str) -> Result<(), polymath_error::Error> {
///         // If URL is Instagram, DO NOT crawl it!
///         if url.contains("instagram.com") {
///             return Err(
///                 polymath_error::Error::new(
///                     polymath_error::ErrorType::Unspecified,
///                     None,
///                     None,
///                 )
///             );
///         }
///     
///         // Otherwise, crawl it!
///         Ok(())
///     }
///
///     fn after_request(
///         &self,
///         _title: &str,
///         _meta: Vec<Meta>,
///         _html: &str,
///     ) -> Result<(), polymath_error::Error> {
///         // Process or analyze the HTML content here.
///         // You can also save result on a database.
///        Ok(())
///     }
/// }

/// ```
pub trait Event: Debug + Send + Sync {
    /// Called before a URL request is made.
    ///
    /// This method is used to perform actions or checks before initiating the crawl of a URL.
    /// If an error is returned, the crawling process for that URL is halted.
    fn before_request(&self, url: &str) -> Result<(), polymath_error::Error>;
    /// Called after a URL has been crawled.
    ///
    /// This method is triggered once the content of a URL has been fetched. It allows you to
    /// process or log the content of the page. Errors returned by this method do not affect
    /// subsequent events or the crawling process for URLs found on the page.
    fn after_request(
        &self,
        title: &str,
        meta: Vec<extractor::meta::Meta>,
        html: &str,
    ) -> Result<(), polymath_error::Error>;
}

/// The [Crawler] struct encapsulates the core functionality of a web crawler.
#[derive(Default, Debug)]
pub struct Crawler {
    allowed_domains: Vec<String>,
    depth: LRUCache<String, usize>,
    events: Vec<Box<dyn Event>>,
    extensions: Vec<String>,
    follow_redirects: bool,
    headers: HashMap<String, String>,
    max_depth: Option<usize>,
    _queue: Vec<String>, // Use polymath-queue later.
    retry_after: u64,
    retry_count: usize,
    timeout: u64,
    user_agent: String,
}

impl Crawler {
    /// Create a [Crawler] to go on pages.
    pub fn new() -> Self {
        Crawler {
            extensions: ALLOWED_EXT.iter().map(|e| e.to_string()).collect(),
            timeout: 10,
            retry_after: 10,
            retry_count: 3,
            follow_redirects: true,
            user_agent: format!("polymath/{}", env!("CARGO_PKG_VERSION")),
            depth: LRUCache::with_capacity(20),
            ..Default::default()
        }
    }

    /// Specifies the list of domains the crawler is allowed to visit.
    /// If the list is empty, the crawler will consider all domains permissible.
    ///
    /// Domains muse use regular expressions patterns
    /// (such as `[\w@:%._+~#=-]{1,256}\.gravitalia(\.com)?$`, to crawl
    /// only gravitalia domains).
    pub fn allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = domains;
        self
    }

    /// Defines the file extensions that the crawler should fetch.
    /// Files with extensions not listed here will be excluded from the crawl.
    pub fn allowed_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    /// Sets whether the crawler should follow HTTP redirects.
    /// If set to false, the crawler stops when it encounters a redirect.
    pub fn follow_redirects(mut self, follow_redirects: bool) -> Self {
        self.follow_redirects = follow_redirects;
        self
    }

    /// Adds a custom HTTP header to be included in each request.
    pub fn add_headers(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Sets a maximum depth for the crawler. The depth is the number of hops
    /// the crawler can make from the starting URL.
    pub fn depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Add new receivers to Crawler [Events](Event).
    pub fn register_event(mut self, event: Box<dyn Event>) -> Self {
        self.events.push(event);
        self
    }

    /// Specifies the number of retry attempts for failed requests
    /// (e.g., due to 4XX, 5XX errors, or timeouts).
    pub fn retry(mut self, retry_count: usize) -> Self {
        self.retry_count = retry_count;
        self
    }

    /// Sets the delay between retry attempts for failed requests.
    pub fn retry_after(mut self, duration: Duration) -> Self {
        self.retry_after = duration.as_secs();
        self
    }

    /// Sets the timeout duration for each request. If a response is not received
    /// within this time, the request is considered to have failed.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration.as_secs();
        self
    }

    /// Sets a custom user agent string for the crawler. This is used in the HTTP
    /// request headers to identify the client making the requests.
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    fn pre_process(&self, url: &str) -> Result<(), polymath_error::Error> {
        for event in &self.events {
            event.before_request(&url)?;
        }

        Ok(())
    }

    fn create_agent(&self, url: &str) -> (Agent, Request) {
        debug!("Creating HTTP agent to perform request.");

        let agent: Agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(self.timeout))
            .redirects(if self.follow_redirects { 3 } else { 0 })
            .user_agent(&self.user_agent)
            .build();

        let request = agent.get(url);
        let request = self
            .headers
            .iter()
            .fold(request, |req, (key, value)| req.set(key, value));

        (agent, request)
    }

    fn post_process(
        &mut self,
        agent: &Agent,
        url: String,
        meta: Vec<Meta>,
        body: &str,
    ) -> Result<(), polymath_error::Error> {
        for event in &self.events {
            // We do not care about result here.
            let _ = event.after_request("", meta.clone(), body);
        }

        for link in extractor::link::find_all_links(body) {
            let depth = *self.depth.get(&url).unwrap_or(&0);
            self.depth.update(&url, depth + 1);

            if let Some(depth) = self.max_depth {
                if self.depth.get(&url).unwrap_or(&0) >= &depth {
                    break;
                }
            }

            debug!("Found {} URL on {}", link, url);
            self.internal_fetch(url.clone(), &agent, link)?;
        }

        Ok(())
    }

    /// Just fetch one page and return its content.
    /// Disabling `pre_process` enables crawling of any page, regardless of options and extensions.
    pub fn just_fetch(
        &self,
        url: String,
        pre_process: bool,
        post_process: bool,
    ) -> Result<String, polymath_error::Error> {
        if pre_process {
            self.pre_process(&url)?;

            if !self.allowed_domains.is_empty() && self.test_domain(&url) {
                return Err(
                    polymath_error::Error::new(
                        polymath_error::ErrorType::Crawler(CrawlerError::InvalidDomain),
                        None,
                        Some(
                            format!(
                                "You have specified a domain limit ({:?}) and {} is not one of them.",
                                self.allowed_domains,
                                url
                            )
                        )
                    )
                );
            }
        }

        let (_, request) = self.create_agent(&url);

        debug!("Fetch {} using the agent.", url);
        let body = request
            .call()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::NetworkError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?
            .into_string()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::ParseError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?;

        if post_process {
            let meta = extractor::meta::extract_meta_tags(&body)?;

            for event in &self.events {
                let _ = event.after_request("", meta.clone(), &body);
            }
        }

        Ok(body)
    }

    /// Crawl a page and extract its substantifique moelle.
    pub fn fetch(&mut self, url: String) -> Result<(), polymath_error::Error> {
        self.pre_process(&url)?;

        if !self.allowed_domains.is_empty() && self.test_domain(&url) {
            return Err(
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(CrawlerError::InvalidDomain),
                    None,
                    Some(
                        format!(
                            "You have specified a domain limit ({:?}) and {} is not one of them.",
                            self.allowed_domains,
                            url
                        )
                    )
                )
            );
        }

        let (agent, request) = self.create_agent(&url);

        self.depth.put(url.clone(), 0);

        debug!("Fetch {} using the agent.", url);

        let body = request
            .call()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::NetworkError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?
            .into_string()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::ParseError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?;
        let meta = extractor::meta::extract_meta_tags(&body)?;
        self.post_process(&agent, url, meta, &body)?;

        Ok(())
    }

    fn internal_fetch(
        &mut self,
        from: String,
        agent: &ureq::Agent,
        url: String,
    ) -> Result<(), polymath_error::Error> {
        self.pre_process(&url)?;

        if !self.allowed_domains.is_empty() && self.test_domain(&url) {
            return Err(
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(CrawlerError::InvalidDomain),
                    None,
                    Some(
                        format!(
                            "You have specified a domain limit ({:?}) and {} is not one of them.",
                            self.allowed_domains,
                            url
                        )
                    )
                )
            );
        }

        let mut request = agent.get(&url);

        for (key, value) in &self.headers {
            request = request.clone().set(key, value);
        }

        debug!("Fetch {} using the agent.", url);
        let body = request
            .call()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::NetworkError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?
            .into_string()
            .map_err(|e| {
                polymath_error::Error::new(
                    polymath_error::ErrorType::Crawler(
                        CrawlerError::ParseError,
                    ),
                    Some(Box::new(e)),
                    None,
                )
            })?;

        let meta = extractor::meta::extract_meta_tags(&body)?;

        self.post_process(&agent, from, meta, &body)?;

        Ok(())
    }

    fn test_domain(&self, url: &str) -> bool {
        url::Url::parse(url)
            .map(|url| {
                url.host_str()
                    .map(|host| {
                        self.allowed_domains.iter().any(|domain| {
                            if let Ok(regex) = Regex::new(domain) {
                                regex.is_match(host)
                            } else {
                                error!(
                                    regex = domain,
                                    "Regex is not a valid expression."
                                );
                                false
                            }
                        })
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }
}
