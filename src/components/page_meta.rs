use dioxus::prelude::*;

pub const BASE_URL: &str = "https://justinsengineering.com";
pub const SITE_NAME: &str = "Justin's Engineering Services";
const OG_IMAGE: &str = "https://justinsengineering.com/images/og.png";
const OG_IMAGE_ALT: &str = "Justin's Engineering Services: Open-Source IoT & Embedded Rust";

/// Renders the per-page `<head>` metadata: title, description, canonical URL,
/// and Open Graph / Twitter card tags. `path` is the route path with a leading
/// slash (e.g. "/" or "/projects").
#[component]
pub fn PageMeta(title: String, description: String, path: String) -> Element {
  let url = format!("{BASE_URL}{path}");
  rsx! {
    document::Title { "{title}" }
    document::Meta { name: "description", content: "{description}" }
    document::Link { rel: "canonical", href: "{url}" }
    document::Meta { property: "og:title", content: "{title}" }
    document::Meta { property: "og:description", content: "{description}" }
    document::Meta { property: "og:url", content: "{url}" }
    document::Meta { property: "og:type", content: "website" }
    document::Meta { property: "og:site_name", content: SITE_NAME }
    document::Meta { property: "og:locale", content: "en_US" }
    document::Meta { property: "og:image", content: OG_IMAGE }
    document::Meta { property: "og:image:width", content: "1200" }
    document::Meta { property: "og:image:height", content: "630" }
    document::Meta { property: "og:image:alt", content: OG_IMAGE_ALT }
    document::Meta { name: "twitter:card", content: "summary_large_image" }
    document::Meta { name: "twitter:title", content: "{title}" }
    document::Meta { name: "twitter:description", content: "{description}" }
    document::Meta { name: "twitter:image", content: OG_IMAGE }
  }
}
