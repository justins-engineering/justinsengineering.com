#![forbid(unsafe_code)]

use dioxus::prelude::*;
use views::{Index, PageNotFound, Projects, Wrapper};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
  #[layout(Wrapper)]
    #[route("/")]
    Index {},
    #[route("/projects")]
    Projects {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
  // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
  Ok(
    Route::static_routes()
      .iter()
      .map(ToString::to_string)
      .collect(),
  )
}

fn main() {
  // dioxus::launch(App);
  dioxus::LaunchBuilder::new()
    // Set the server config only if we are building the server target
    .with_cfg(server_only! {
        ServeConfig::builder()
            // Enable incremental rendering
            .incremental(
                IncrementalRendererConfig::new()
                    // Store static files in the public directory where other static assets like wasm are stored
                    .static_dir(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("public")
                    )
                    // Don't clear the public folder on every build. The public folder has other files including the wasm
                    // binary and static assets required for the app to run
                    .clear_cache(false)
            )
            .enable_out_of_order_streaming()
    })
    .launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-light.ico"),
      sizes: "32x32",
    }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-light.ico"),
      sizes: "32x32",
      media: "prefers-color-scheme: light",
    }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-dark.ico"),
      sizes: "32x32",
      media: "prefers-color-scheme: dark",
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-light.svg"),
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-light.svg"),
      media: "prefers-color-scheme: light",
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-dark.svg"),
      media: "prefers-color-scheme: dark",
    }
    Router::<Route> {}
  }
}
