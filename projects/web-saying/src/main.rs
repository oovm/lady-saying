#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]

use dioxus::prelude::*;
use lady_saying::LadySaying;
mod components;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = LadySaying::builtin()[0].clone();
    cx.render(rsx!(
        div {
            "dioxus"
        }
        SayingComponent { lady: data }
    ))
}




