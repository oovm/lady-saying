#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]

use dioxus::prelude::*;

mod components;

pub use self::components::{GithubLink, Saying, LadySaying};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = LadySaying::builtin()[0].clone();
    cx.render(rsx!(
        Saying { lady: data }
    ))
}




