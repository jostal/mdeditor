extern crate dioxus;
extern crate dioxus_html_macro;
extern crate comrak;
use dioxus::prelude::*;

mod ui;

fn main() {

    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            w.with_maximized(true).with_title("mdeditor")
        })
    });
}

static CONTENT: Atom<String> = |_| "".to_string();

fn app(cx: Scope) -> Element {
    use_init_atom_root(&cx);

    cx.render(rsx!{
        div {
            style { [include_str!("./global.css")] }
            ui::ui {} 
        }
        
    })
}