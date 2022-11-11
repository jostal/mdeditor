use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct NewPageButtonProps {}
pub fn new_page_button(cx: Scope<NewPageButtonProps>) -> Element {
    cx.render(rsx!{
        div {
            button {
                class: "btn-new-page",
                b{"NEW PAGE +"}
            }
        }
    })
}