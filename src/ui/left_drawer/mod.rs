use dioxus::prelude::*;

mod new_page;
mod open_folder;

#[derive(Props, PartialEq)]
pub struct LeftDrawerProps {

}

pub fn left_drawer(cx: Scope<LeftDrawerProps>) -> Element {
    cx.render(rsx!{
        div {
            class: "leftDrawer",
            style { [include_str!("./leftDrawer.css")]}
            open_folder::open_folder_button {}
            new_page::new_page_button {}
        }
    })
}