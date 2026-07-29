use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
  rsx! {
    document::Title { "Page Not Found | Justin's Engineering Services" }
    document::Meta { name: "robots", content: "noindex" }
    div { class: "text-center flex-1 max-w-none",
      h1 { class: "text-9xl my-12", "404" }
      h2 { class: "my-8", "Oops! Page not found." }
      h3 { class: "font-light my-8",
        "The page {route:?} might have been removed or is temporarily unavailable."
      }
      Link { to: Route::Index {}, class: "btn btn-primary my-8", "Go Home" }
    }
  }
}
