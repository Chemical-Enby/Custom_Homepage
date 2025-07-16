mod image_response;
use crate::image_response::ImageResponse;

use std::ffi::{OsStr, OsString};
use dioxus::dioxus_core::DynamicNode::Component;
use dioxus::document::DocumentContext;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const EXAMPLE_PHOTO: Asset = asset!("/assets/Laced.jpg", ImageAssetOptions::new().with_avif());

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Title { "Test" }

        image_view {}
    }
}

#[component]
fn image_view() -> Element {
    let skip = move |evt| {};
    let save = move |evt| {};
    let img_src = use_hook(|| EXAMPLE_PHOTO);

    rsx! {
        div { id: "main",
            div { id: "img-wrapper",
                img { src: "{img_src}" }
            }

            div { id: "buttons",
                button { onclick: skip, id: "skip", "skip" }
                button { onclick: save, id: "save", "save" }
            }
        }
    }
}
