use dioxus::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};


pub fn parser(content: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;

    markdown_to_html(content, &options)
}
