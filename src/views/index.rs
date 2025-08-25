use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
  rsx! {
    div { class: "hero flex-1",
      div { class: "hero-content text-center",
        div { class: "max-w-md",
          h1 { class: "text-5xl font-bold",
            "Remote"
            span { class: "text-accent", " IoT " }
            "for the masses"
          }
          p { class: "py-6",
            "Building affordable remote IoT solutions for everyone, from hobbyists to enterprise."
          }
          a {
            class: "btn btn-primary",
            href: "https://github.com/justins-engineering",
            "Projects"
          }
        }
      }
    }
  }
}
