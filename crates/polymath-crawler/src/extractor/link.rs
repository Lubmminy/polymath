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
