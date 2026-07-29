use crate::Route;
use crate::components::PageMeta;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdCloudCog, LdCodeXml, LdCpu, LdGithub};

const JSON_LD: &str = r##"{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "ProfessionalService",
      "@id": "https://justinsengineering.com/#organization",
      "name": "Justin's Engineering Services, LLC",
      "alternateName": "JES",
      "url": "https://justinsengineering.com/",
      "logo": "https://justinsengineering.com/images/og.png",
      "image": "https://justinsengineering.com/images/og.png",
      "email": "justin@jes.contact",
      "description": "Solo engineering consultancy building PidgeIoT, an open-source IoT device management platform, and consulting in Zephyr embedded firmware and full-stack Rust.",
      "founder": {
        "@type": "Person",
        "name": "Justin Forgue",
        "jobTitle": "Founder & Principal"
      },
      "address": {
        "@type": "PostalAddress",
        "addressRegion": "MA",
        "addressCountry": "US"
      },
      "areaServed": "Worldwide",
      "sameAs": ["https://github.com/justins-engineering"],
      "knowsAbout": [
        "Embedded firmware",
        "Zephyr RTOS",
        "Rust",
        "IoT device management",
        "Nordic nRF9160",
        "Nordic nRF9151",
        "ESP32",
        "Cloudflare Workers",
        "WebAssembly",
        "Over-the-air firmware updates"
      ]
    },
    {
      "@type": "WebSite",
      "@id": "https://justinsengineering.com/#website",
      "url": "https://justinsengineering.com/",
      "name": "Justin's Engineering Services",
      "publisher": { "@id": "https://justinsengineering.com/#organization" }
    },
    {
      "@type": "SoftwareApplication",
      "name": "PidgeIoT",
      "url": "https://pidgeiot.com",
      "applicationCategory": "DeveloperApplication",
      "operatingSystem": "Web",
      "description": "Open-source (AGPL-3.0) IoT device management platform: device provisioning, device shadows, telemetry, alerts, and over-the-air firmware updates.",
      "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD" },
      "license": "https://www.gnu.org/licenses/agpl-3.0.html",
      "publisher": { "@id": "https://justinsengineering.com/#organization" }
    }
  ]
}"##;

#[component]
pub fn Index() -> Element {
  rsx! {
    PageMeta {
      title: "Justin's Engineering Services | IoT & Embedded Rust",
      description: "Builders of PidgeIoT, an open-source IoT device management platform. Consulting in Zephyr embedded firmware and full-stack Rust, from device to cloud.",
      path: "/",
    }
    document::Script { r#type: "application/ld+json", {JSON_LD} }
    div { class: "flex-1 py-8 px-2 lg:px-8",
      div { class: "hero py-12 lg:py-20",
        div { class: "hero-content text-center flex-col items-stretch justify-around",
          h1 {
            "Open-Source "
            span { class: "text-accent", "IoT" }
            " Platforms and "
            span { class: "text-accent", "Embedded" }
            " Engineering"
          }
          p { class: "text-balance",
            "We build "
            a { class: "link", href: "https://pidgeiot.com", "PidgeIoT" }
            ", an open-source IoT device management platform, and take on consulting work in
            embedded firmware and full-stack Rust — from Zephyr device bring-up on real hardware
            to edge-cloud backends."
          }
          div { class: "flex flex-col lg:flex-row justify-center-safe gap-3",
            a {
              class: "btn btn-primary lg:w-3/16",
              href: "https://pidgeiot.com",
              aria_label: "PidgeIoT",
              "PidgeIoT"
            }
            Link {
              class: "btn btn-secondary lg:w-3/16",
              to: Route::Projects {},
              aria_label: "projects",
              "Projects"
            }
          }
        }
      }

      section { id: "services", class: "py-8",
        h2 { class: "text-5xl font-bold", "Services" }
        div { class: "divider mt-0" }

        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdCpu,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Embedded Firmware & IoT Devices" }
              p {
                "Zephyr RTOS firmware for cellular and WiFi devices: Nordic nRF9160 and nRF9151
                (LTE-M, GNSS, FOTA over LTE) and ESP32, including the WiFi-connected ESP32-C6.
                Device connectivity over HTTPS, WebSocket, and CoAP; device provisioning and
                fleet authentication; hardware bring-up and verification on real devices."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdCloudCog,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Edge-Cloud Rust Platforms" }
              p {
                "Full-stack Rust services on Cloudflare Workers, Durable Objects, Queues, and R2,
                with WebAssembly frontends built in Dioxus. Postgres data layers and self-hosted
                Ory Kratos authentication."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdCodeXml,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Open Source" }
              p {
                "Our work is developed in the open: PidgeIoT is AGPL-3.0 licensed, the "
                a {
                  class: "link",
                  href: "https://github.com/justins-engineering/pigeon",
                  "pigeon"
                }
                " Zephyr device library and its "
                a {
                  class: "link",
                  href: "https://github.com/justins-engineering/pigeon-examples",
                  "examples"
                }
                " are on GitHub, and we maintain the "
                a {
                  class: "link",
                  href: "https://crates.io/crates/ory-kratos-client-wasm",
                  "ory-kratos-client-wasm"
                }
                " crate."
              }
            }
          }
        }
      }

      section { id: "pidgeiot", class: "py-8",
        h2 { class: "text-5xl font-bold", "PidgeIoT" }
        div { class: "divider mt-0" }

        div { class: "card bg-base-200 my-6",
          div { class: "card-body",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title mb-4", "IoT device management, without the lock-in" }
              p { class: "my-4",
                "PidgeIoT is our flagship product: an open-source (AGPL-3.0) IoT device management
                platform, publicly launched in July 2026 and free while in beta."
              }
              ul { class: "list-outside list-disc pl-8",
                li { "Device provisioning with per-device Ed25519 keypairs and compact binary tokens" }
                li { "Configuration push via device shadows" }
                li { "Telemetry with queryable history" }
                li { "Email alerts" }
                li { "Over-the-air firmware updates" }
              }
              p { class: "my-4",
                "Built entirely in Rust: the backend runs on Cloudflare Workers and Durable
                Objects, and the dashboard is a Dioxus WebAssembly app."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-primary",
                href: "https://pidgeiot.com",
                "Visit pidgeiot.com"
              }
              a {
                class: "btn btn-soft",
                href: "https://pidgeiot.com/getting-started",
                "Getting Started"
              }
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/pidgeiot",
                aria_label: "PidgeIoT on GitHub",
                Icon { icon: LdGithub }
              }
            }
          }
        }
      }

      section { id: "about", class: "py-8",
        h2 { class: "text-5xl font-bold", "About" }
        div { class: "divider mt-0" }
        p { class: "my-4 text-balance",
          "Justin's Engineering Services, LLC is the solo consultancy of Justin Forgue,
          Founder & Principal, based in western Massachusetts, US. For project inquiries, "
          a { class: "link", href: "mailto:justin@jes.contact", "get in touch" }
          "."
        }
      }
    }
  }
}
