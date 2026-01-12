use web_sys::wasm_bindgen::{self, prelude::*};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from WASM! This is example-wasm.");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! (from example-wasm)", name)
}
