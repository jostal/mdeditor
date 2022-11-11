use dioxus::prelude::*;
use crate::ui::file_explorer;
use CONTENT;

#[derive(PartialEq, Props)]
pub struct OpenFolderButtonProps {}
pub fn open_folder_button(cx: Scope<OpenFolderButtonProps>) -> Element {
    let open_file_explorer = use_state(&cx, || false);
    let set_content = use_set(&cx, CONTENT);

    cx.render(rsx!{
        div {
            button {
                class: "btn-open-folder",
                onclick: move |_| open_file_explorer.set(true),
                b{"OPEN FOLDER"}
            }
            if **(open_file_explorer) {
                let content = file_explorer::file_explorer_ui();
                set_content(content);
                cx.render(rsx!{div{}})
            } else {
                cx.render(rsx!{div{}}) // render nothing
            }
        }
    })
}
