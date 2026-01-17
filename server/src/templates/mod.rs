use std::sync::atomic::{AtomicU64, Ordering};

use maud::{DOCTYPE, Markup, PreEscaped, html};
use shared::wasm;

static COUNT: AtomicU64 = AtomicU64::new(0);

pub async fn index() -> Markup {
    html! { (DOCTYPE) html lang="en" { head {
        title { "cmrnw" }
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1" {}
        meta name="color-scheme" content="light dark" {}
        link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.sand.min.css" {}
        script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" {}
        script { (wasm!("example_wasm")) }
    }
        (main())
    }}
}

fn main() -> Markup {
    html! { body.container style="display:flex; flex-direction:column; height:100vh" {
        header { hgroup {
            h1 { "cmrnw" }
            h6 { "0x68656C6C6F20776F726C64" }
        }}
        main style="display:flex; flex-grow:1" {
            aside style="width: 10%" {
                nav { ul {
                li { a { "nav_a" } }
                li { a { "nav_b" } }
                li { a { "nav_c" } }
            }}}
            #content style="flex-direction:column" {
                button hx-post="/clicked" hx-swap="outerHTML" { "Click Me" }
            }
        }
        footer style="height:10%" { a href="https://github.com/cmrnwlsh/cmrnw" { "source" } }
    }}
}

pub async fn clicked() -> Markup {
    html! {
        button hx-post="/clicked" hx-swap="outerHTML" {
            "Click Me: " (COUNT.fetch_add(1, Ordering::Relaxed))
        }
    }
}
