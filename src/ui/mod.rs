use dioxus::prelude::*;

mod left_drawer;
mod file_explorer;
mod editor;

#[derive(Props, PartialEq)]
pub struct UIProps {

}

pub fn ui(cx: Scope<UIProps>) -> Element {

    cx.render(rsx!{
        div {
            class: "container-ui",
            style {[include_str!("./ui.css")]}
            left_drawer::left_drawer {}
            div {
                class: "container-editor",
                editor::editor {}
            }
        }
    })
}