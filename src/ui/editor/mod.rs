use dioxus::prelude::*;

use CONTENT;

mod parser;

#[derive(Props, PartialEq)]
pub struct EditorProps {

}

pub fn editor(cx: Scope<EditorProps>) -> Element {
    let content = use_read(&cx, CONTENT);

    let html = parser::parser(content);

    cx.render(rsx!{
        div {
            class: "editor",
            style { [include_str!("./editor.css")]}
            div {
                dangerous_inner_html: "{html}",
            }
        }
    })
}