use regex_lite::Regex;

lazy_static! {
    static ref URL: Regex = Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
}

pub(crate) fn find_all_links(content: &str) -> Vec<String> {
    let mut urls = Vec::new();

    for mat in URL.find_iter(content) {
        urls.push(mat.as_str().to_string());
    }

    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_extraction() {
        let html = r#"
        <body>
        <p>La vie seule... c'est dur</p>
        <p>Mais avec notre ami imaginaire, c'est mieux : <a href="https://www.gravitalia.com/">cliquez ici</a></p>
        <h1>Voir nos actualités : http://news.gravitalia.com/</h1>
        <script>
        window.open("https://youtu.be/yG-1v-_1NVM", "_blank");
        </script>
        </body>
        "#;

        assert_eq!(
            vec![
                "https://www.gravitalia.com/",
                "http://news.gravitalia.com/",
                "https://youtu.be/yG-1v-_1NVM"
            ],
            find_all_links(html)
        );
    }
}
