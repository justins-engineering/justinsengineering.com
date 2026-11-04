use crate::Route;
use crate::components::PageMeta;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdBusFront, LdCloudCog, LdCpu, LdGithub, LdRadioTower};

const JSON_LD: &str = r##"{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "ProfessionalService",
      "@id": "https://justinsengineering.com/#organization",
      "name": "Justin's Engineering Services LLC",
      "alternateName": "JES",
      "url": "https://justinsengineering.com/",
      "logo": "https://justinsengineering.com/images/og.png",
      "image": "https://justinsengineering.com/images/og.png",
      "email": "ops@jes.contact",
      "telephone": "+1-413-345-6759",
      "description": "Independent engineering consultancy for hire: Zephyr embedded firmware on cellular and Wi-Fi devices, Rust backends on Cloudflare Workers, and IoT device management. Built PidgeIoT, the open-source platform that PVTA's three solar real-time departure signs run on.",
      "founder": {
        "@type": "Person",
        "name": "Justin Forgue",
        "jobTitle": "Principal Engineer"
      },
      "address": {
        "@type": "PostalAddress",
        "addressLocality": "West Springfield",
        "addressRegion": "MA",
        "addressCountry": "US"
      },
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
        "Over-the-air firmware updates",
        "Real-time transit departure signs"
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
      title: "Justin's Engineering Services | Embedded Firmware & Rust Consulting",
      description: "Contract embedded firmware and Rust backend engineering: Zephyr on nRF91 and ESP32, cellular IoT, over-the-air updates, Cloudflare Workers. Built PidgeIoT and the firmware behind PVTA's three solar real-time departure signs.",
      path: "/",
    }
    document::Script { r#type: "application/ld+json", {JSON_LD} }
    main { class: "flex-1 py-8 px-2 lg:px-8",
      div { class: "hero py-12 lg:py-20",
        div { class: "hero-content text-center flex-col items-stretch justify-around",
          h1 {
            span { class: "text-accent", "Embedded" }
            " Firmware and "
            span { class: "text-accent", "Rust" }
            " Engineering, on Contract"
          }
          p { class: "text-balance",
            "We take on firmware, backend and IoT platform work for companies that need it
            built and shipped: Zephyr device bring-up on real cellular and Wi-Fi hardware,
            over-the-air updates that survive the field, and the Rust services devices talk
            to. We built "
            a { class: "link", href: "https://pidgeiot.com", "PidgeIoT" }
            " and the firmware behind PVTA's solar departure signs the same way."
          }
          div { class: "flex flex-col lg:flex-row justify-center-safe gap-3",
            a {
              class: "btn btn-primary lg:w-3/16",
              href: "mailto:ops@jes.contact?subject=Project%20inquiry&body=What%20you%20need%20built%3A%0ATimeline%3A%0A",
              "Start a project"
            }
            Link {
              class: "btn btn-secondary lg:w-3/16",
              to: Route::Projects {},
              aria_label: "projects",
              "Projects"
            }
            a {
              class: "btn btn-accent lg:w-3/16",
              href: "#engagements",
              "How we work"
            }
          }
        }
      }
      section { id: "services", class: "py-8",
        h2 { class: "text-5xl font-bold", "Services" }
        div { class: "divider mt-0" }
        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdCpu,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Embedded firmware" }
              p {
                "Zephyr RTOS firmware in C for cellular and Wi-Fi devices: Nordic nRF9160 and
                nRF9151 (LTE-M, NB-IoT, GNSS) and ESP32, including the ESP32-C6. MCUboot
                updates with download resume, watchdog and fault recovery, power budgets for
                solar and battery, and bring-up and verification on the real board, not a
                simulator. We have shipped firmware that runs unattended at bus stops through
                New England winters."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdRadioTower,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Device connectivity and management" }
              p {
                "The link between a device and its backend: HTTPS, CoAP over DTLS (with
                Connection ID), MQTT with TLS-PSK or certificates, WebSocket push, and
                Verizon NB-IoT non-IP data. Per-device authentication, device shadows for
                configuration push, store-and-forward telemetry, fleet-wide firmware
                rollout. On our open-source "
                a { class: "link", href: "https://pidgeiot.com", "PidgeIoT" }
                " platform or inside yours."
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
              h3 { class: "card-title", "Rust backends" }
              p {
                "Services on Cloudflare Workers, Durable Objects, Queues and R2, with
                PostgreSQL behind them and Dioxus WebAssembly frontends in front. Serverless
                cost engineering, usage metering and billing, self-hosted Ory Kratos
                identity. Native Rust services where the edge is the wrong fit: a DTLS
                terminator and an MQTT broker are both ours and both in production."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              Icon {
                icon: LdBusFront,
                width: 32,
                height: 32,
                class: "text-accent",
              }
              h3 { class: "card-title", "Transit technology" }
              p {
                "Real-time passenger information from the feed to the stop: GTFS and GTFS
                Realtime integration, prediction APIs, and the signs and devices that show
                them. Our principal designed, built and maintains PVTA's three solar
                real-time departure signs as a UMass Transit employee, in service since 2023,
                and has driven and dispatched for the agency."
              }
            }
          }
        }
      }
      section { id: "engagements", class: "py-8",
        h2 { class: "text-5xl font-bold", "How we work" }
        div { class: "divider mt-0" }
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
          div { class: "card bg-base-200",
            div { class: "card-body",
              h3 { class: "card-title", "Contract engineering" }
              p {
                "A senior engineer on your team for a defined stretch, hourly or by the day,
                remote from Massachusetts. Direct or through your staffing agency."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              h3 { class: "card-title", "Fixed-scope projects" }
              p {
                "A board brought up, a firmware feature shipped, a service built: scoped in
                writing, quoted as a fixed fee, delivered with tests and documentation."
              }
            }
          }
          div { class: "card bg-base-200",
            div { class: "card-body",
              h3 { class: "card-title", "Design review and advisory" }
              p {
                "A second pair of eyes on a device architecture, a transport choice, an
                update mechanism or a cloud cost model before you commit to it. Short
                engagements, written findings."
              }
            }
          }
        }
        div { class: "card bg-base-200 my-6",
          div { class: "card-body",
            h3 { class: "text-2xl card-title", "Tell us what you need built" }
            p { class: "my-2",
              "Send a few lines about the device, the stack and the timeline. We reply within
              one business day with whether it fits and what the next step is. No obligation."
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-primary",
                href: "mailto:ops@jes.contact?subject=Project%20inquiry&body=What%20you%20need%20built%3A%0ATimeline%3A%0A",
                "Email ops@jes.contact"
              }
              a { class: "btn btn-soft", href: "tel:+14133456759", "(413) 345-6759" }
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
              h3 { class: "text-2xl card-title mb-4", "Our own platform, and our proof of work" }
              p { class: "my-4",
                "PidgeIoT is the open-source (AGPL-3.0) IoT device management platform we built
                and run, publicly launched in July 2026. It is what our consulting work looks
                like when nobody else sets the scope."
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
                Objects, and the dashboard is a Dioxus WebAssembly app. The "
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
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-primary",
                href: "https://pidgeiot.com",
                aria_label: "PidgeIoT",
                "PidgeIoT"
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
          "Justin's Engineering Services LLC is the independent engineering consultancy of
          Justin Forgue, Principal Engineer, in West Springfield, Massachusetts. Nine years
          in production software, five of them building and operating cellular devices and
          the services behind them."
        }
        p { class: "my-4",
          a { class: "link text-nowrap", href: "tel:+14133456759", "(413) 345-6759" }
          " · "
          a { class: "link", href: "mailto:ops@jes.contact", "ops@jes.contact" }
        }
      }
    }
  }
}
