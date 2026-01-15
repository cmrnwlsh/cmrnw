use std::sync::atomic::{AtomicU64, Ordering};

use maud::{DOCTYPE, Markup, PreEscaped, html};

pub async fn index() -> Markup {
    html! { (DOCTYPE) html lang="en" {
        head {
            meta charset="utf-8" {}
            meta name="viewport" content="width=device-width, initial-scale=1" {}
            meta name="color-scheme" content="light dark" {}
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.sand.min.css" {}
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" {}
            script type="module" { (PreEscaped(concat!(
                "import init from '/static/example-wasm/example_wasm.js';",
                "(async () => await init())()"
            ))) }
            title { "cmrnw" }
        }
        body {
            main.container {
                h1 { "welcome" }
                button hx-post="/clicked" hx-swap="outerHTML" { "Click Me" }
            }
        }
    }}
}

static COUNT: AtomicU64 = AtomicU64::new(0);

pub async fn clicked() -> Markup {
    html! {
        button hx-post="/clicked" hx-swap="outerHTML" {
            "Click Me: " (COUNT.fetch_add(1, Ordering::Relaxed))
        }
    }
}
