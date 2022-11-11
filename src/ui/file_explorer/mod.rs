extern crate dirs;
extern crate native_dialog;

use std::fs;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct FileExplorerProps {

}

pub fn file_explorer_ui() -> String {

    let path = native_dialog::FileDialog::new()
        .set_location("~/Downloads")
        .show_open_single_file()
        .unwrap();

    

    let yes = native_dialog::MessageDialog::new()
        .set_type(native_dialog::MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();
    
    if yes {
        let contents = fs::read_to_string(path.unwrap().to_str().unwrap().to_string()).expect("Unable to read file");
        contents
    } else {
        "".to_string()
    }
}
