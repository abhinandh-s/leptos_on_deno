use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Hello from Leptos 0.8.2 running on Deno!"</h1>
        </main>
    }
}

#[cfg(feature = "ssr")]
mod server {
    use super::*;
    use leptos::{LeptosOptions, render_to_string};

    #[wasm_bindgen]
    pub async fn render() -> String {
        let options = LeptosOptions {
            site_root: ".".into(),
            site_pkg_dir: "pkg".into(),
            site_addr: None,
            reload_port: None,
            env: None,
            ..Default::default()
        };

        render_to_string(move || {
            provide_context(options.clone());
            view! { <App /> }
        })
        .await
    }
}
