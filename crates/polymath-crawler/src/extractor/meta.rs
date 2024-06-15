//! Meta tag extraction.

use polymath_error::{Error, ErrorType::Scraper, ScraperError};
use scraper::{Html, Selector};

/// Representation of data contained in a
/// [`<meta>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta) tag.
#[derive(Debug, Default, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Meta {
    /// A non-standard attribute used primarily
    /// with the [Open Graph](https://ogp.me/) protocol.
    pub property: Option<String>,
    /// Metadata name.
    pub name: Option<String>,
    /// Metadata value containing value for the
    /// `http-equiv` or `name` attribute.
    pub content: Option<String>,
    /// Document's character encoding.
    pub charset: Option<String>,
    /// Refer to
    /// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta#http-equiv).
    pub http_equiv: Option<String>,
    /// Specify a scheme to interpret the property’s value.
    pub scheme: Option<String>,
}

/// Extracts the [`<meta>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
/// tags contained in a complete HTML page.
pub fn extract_meta_tags(body: &str) -> Result<Vec<Meta>, Error> {
    let mut metas: Vec<Meta> = Vec::new();

    let document = Html::parse_document(body);
    let selector = Selector::parse("meta").map_err(|_| {
        Error::new(
            Scraper(ScraperError::Selector),
            None,
            Some("while getting meta tags".to_owned()),
        )
    })?;

    for element in document.select(&selector) {
        metas.push(Meta {
            name: element.value().attr("name").map(str::to_owned),
            property: element.value().attr("property").map(str::to_owned),
            content: element.value().attr("content").map(str::to_owned),
            charset: element.value().attr("charset").map(str::to_owned),
            http_equiv: element.value().attr("http-equiv").map(str::to_owned),
            scheme: element.value().attr("scheme").map(str::to_owned),
        });
    }

    Ok(metas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_extraction() {
        let html = r#"
        <!DOCTYPE html><html lang="fr"><head>
        <meta charset="utf-8">
        <title>RIP mon coeur</title>
        <link rel="stylesheet" href="/julien.css">
        <meta property="og:type" content="website">
        <meta property="description" content="Mon coeur a été brisé par un M. C... ?!">
        </head>
        <body>
        <p>j'vais cabler</p>
        </body>
        "#;

        assert_eq!(
            vec![
                Meta {
                    charset: Some("utf-8".to_owned()),
                    ..Default::default()
                },
                Meta {
                    property: Some("og:type".to_owned()),
                    content: Some("website".to_owned()),
                    ..Default::default()
                },
                Meta {
                    property: Some("description".to_owned()),
                    content: Some(
                        "Mon coeur a été brisé par un M. C... ?!".to_owned()
                    ),
                    ..Default::default()
                }
            ],
            extract_meta_tags(html).unwrap()
        )
    }
}
