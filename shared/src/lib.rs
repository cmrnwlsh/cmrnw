#[macro_export]
macro_rules! wasm {
    ($mod:literal) => {
        (PreEscaped(concat!(
            "import('/static/",
            $mod,
            "/",
            $mod,
            ".js').then(mod => mod.default())"
        )))
    };
}
