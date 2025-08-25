use crate::components::{Logo, ThemeController};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdGithub;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    header { class: "navbar shadow-sm bg-base-200",
      div { class: "flex-1", Logo {} }
      nav { class: "flex items-center justify-between py-2 gap-x-2",
        a {
          class: "link link-hover mr-1",
          href: "mailto:justin@jes.contact",
          "Contact"
        }
        a { href: "https://github.com/justins-engineering",
          Icon { icon: LdGithub }
        }
        div { class: "divider divider-horizontal mx-0" }
        ThemeController {}
      }
    }
  }
}
