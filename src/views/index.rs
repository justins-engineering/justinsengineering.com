use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
  rsx! {
    div { class: "hero flex-1",
      div { class: "hero-content text-center flex-col items-stretch h-full lg:h-7/10 justify-around",
        h1 {
          "Novel "
          span { class: "text-accent", "Cellular" }
          " Systems and "
          span { class: "text-accent", "IoT" }
          " Solutions"
        }
        p { class: "text-balance",
          "Building affordable, open-source IoT solutions for everyone — from hobbyists to enterprises. We explore cutting-edge technologies while prioritizing security, reliability, and ease of use."
        }
        div { class: "flex flex-col lg:flex-row justify-center-safe gap-3",
          Link {
            class: "btn btn-primary lg:w-3/16",
            to: Route::Projects {},
            aria_label: "projects",
            "Projects"
          }
                // Link {
        //   to: Route::About {},
        //   class: "btn btn-secondary lg:w-3/16",
        //   aria_label: "about",
        //   "About"
        // }
        }
      }
    }
  }
}
