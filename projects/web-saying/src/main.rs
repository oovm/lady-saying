#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]

use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod components;

pub use self::components::{GithubLink, Saying, LadySaying};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = LadySaying::builtin().choose(&mut thread_rng()).unwrap().clone();
    // let data = use_state(&cx, || raw.clone());
    cx.render(rsx!(
        Saying { lady: data }
    ))
}




