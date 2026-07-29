use crate::components::PageMeta;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdContainer, LdGithub, LdGlobe, LdPackage};

#[component]
fn Tags(tags: Vec<&'static str>) -> Element {
  rsx! {
    div { class: "flex flex-wrap gap-2 my-1",
      for tag in tags {
        span { class: "badge badge-outline", "{tag}" }
      }
    }
  }
}

#[component]
pub fn Projects() -> Element {
  rsx! {
    PageMeta {
      title: "Projects | Justin's Engineering Services",
      description: "Open-source software and hardware projects: PidgeIoT device management, the pigeon Zephyr library, Rust WASM clients, and custom nRF9160 hardware designs.",
      path: "/projects",
    }
    div { class: "flex-1 py-8 px-2 lg:px-8",
      header { class: "pb-4",
        h1 { "Projects" }
        p { class: "my-4 text-balance",
          "Open-source software and hardware we build and maintain — everything here lives on "
          a { class: "link", href: "https://github.com/justins-engineering", "GitHub" }
          ". Start with PidgeIoT, our flagship platform."
        }
      }

      section { id: "software", class: "py-4",
        h2 { class: "text-3xl font-bold", "Software" }
        div { class: "divider mt-0" }

        div { class: "card bg-base-200 my-6",
          div { class: "card-body",
            article { class: "flex-1",
              div { class: "flex flex-wrap items-center gap-3",
                h3 { class: "text-2xl card-title", "PidgeIoT" }
                span { class: "badge badge-accent", "Flagship" }
                span { class: "badge badge-outline", "Free while in beta" }
              }
              p { class: "text-lg font-medium mt-2", "IoT device management, without the lock-in." }
              Tags { tags: vec!["Rust", "Cloudflare Workers", "Dioxus / WASM", "AGPL-3.0"] }
              p { class: "my-2",
                "Provision devices with per-device Ed25519 keys, push configuration through
                device shadows, collect queryable telemetry, send email alerts, and ship
                over-the-air firmware updates — all from one open-source platform."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-primary",
                href: "https://pidgeiot.com",
                Icon { icon: LdGlobe }
                "pidgeiot.com"
              }
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/pidgeiot",
                Icon { icon: LdGithub }
                "GitHub"
              }
            }
          }
        }

        div { class: "card bg-base-200 my-6",
          div { class: "card-body",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Pigeon Device Library" }
              p { class: "text-lg font-medium mt-2", "Connect Zephyr firmware to PidgeIoT." }
              Tags {
                tags: vec![
                  "Zephyr RTOS",
                  "nRF9160 / nRF9151",
                  "ESP32-C6",
                  "HTTPS · WebSocket · CoAP",
                ],
              }
              p { class: "my-2",
                "Device shadows, telemetry, device logs, and firmware updates for your own
                firmware. The companion examples repository has ready-to-build samples for
                Nordic cellular boards, the ESP32-C6, and Zephyr's native_sim — no hardware
                required to try it."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/pigeon",
                Icon { icon: LdGithub }
                "pigeon"
              }
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/pigeon-examples",
                Icon { icon: LdGithub }
                "pigeon-examples"
              }
            }
          }
        }

        div { class: "card bg-base-200 my-6",
          div { class: "card-body",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Kratos Client Rust" }
              p { class: "text-lg font-medium mt-2",
                "An Ory Kratos API client that works in the browser."
              }
              Tags { tags: vec!["Rust", "WASM", "crates.io"] }
              p { class: "my-2",
                "A maintained fork of Ory's generated Kratos client that swaps reqwest for the
                browser's native Fetch API when targeting WebAssembly. Published on crates.io
                as ory-kratos-client-wasm."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/kratos-client-rust",
                Icon { icon: LdGithub }
                "GitHub"
              }
              a {
                class: "btn btn-soft",
                href: "https://crates.io/crates/ory-kratos-client-wasm",
                Icon { icon: LdPackage }
                "crates.io"
              }
            }
          }
        }

        div { class: "card lg:card-side bg-base-200 my-6",
          figure { class: "lg:w-1/2",
            img {
              src: asset!("assets/images/kratos-selfservice-wasm.png"),
              alt: "Screenshot of the Kratos Selfservice WASM sign-in user interface",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Kratos Selfservice WASM" }
              p { class: "text-lg font-medium mt-2", "Kratos self-service UI, fully client-side." }
              Tags { tags: vec!["Dioxus", "Tailwind CSS", "daisyUI", "Docker"] }
              p { class: "my-2",
                "A single-page recreation of Ory's "
                a {
                  class: "link",
                  href: "https://github.com/ory/kratos-selfservice-ui-node",
                  "kratos-selfservice-ui-node"
                }
                " with all runtime code compiled to WebAssembly, built on our
                ory-kratos-client-wasm crate (above). Session state combines Dioxus
                signals with a session-expiry cookie for persistence."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/justins-engineering/kratos-selfservice-wasm",
                Icon { icon: LdGithub }
                "GitHub"
              }
              a {
                class: "btn btn-soft",
                href: "https://hub.docker.com/r/jeseng/kratos-selfservice-wasm",
                Icon { icon: LdContainer }
                "Docker Hub"
              }
            }
          }
        }

        div { class: "card lg:card-side lg:flex-row-reverse bg-base-200 my-6",
          figure { class: "lg:w-1/2 lg:rounded-r-lg! lg:rounded-l-none!",
            img {
              src: asset!("assets/images/departure-sign.jpg"),
              alt: "Solar-powered cellular departure board showing real-time bus departures",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Real-time Cellular Departure Board" }
              p { class: "text-lg font-medium mt-2",
                "A solar-powered transit sign, updated in real time over LTE."
              }
              Tags { tags: vec!["Zephyr RTOS", "nRF9160", "MCUboot", "FOTA"] }
              ul { class: "list-outside list-disc pl-8 my-2",
                li { "Zephyr RTOS with the open-source MCUboot bootloader" }
                li { "Firmware built, signed, and released automatically on GitHub" }
                li {
                  "Remote updates downloaded over the cellular network and verified
                  on-device before install"
                }
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/embedded-departure-board",
                Icon { icon: LdGithub }
                "GitHub"
              }
            }
          }
        }
      }

      section { id: "hardware", class: "py-8",
        h2 { class: "text-3xl font-bold", "Hardware" }
        div { class: "divider mt-0" }

        div { class: "card lg:card-side bg-base-200 my-6",
          figure { class: "lg:w-1/2",
            img {
              src: asset!("assets/images/neopixel-6-display-controller.png"),
              alt: "Neopixel 6 display controller circuit board render",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Neopixel 6 Display Controller" }
              p { class: "text-lg font-medium mt-2", "One controller board, six NZR displays." }
              Tags { tags: vec!["Feather form factor", "WS28xx / NZR"] }
              p { class: "my-2",
                "Drives up to six NZR (neopixel / WS28xx) displays from any "
                a {
                  class: "link",
                  href: "https://learn.adafruit.com/adafruit-feather/feather-specification",
                  "Adafruit Feather compatible"
                }
                " MCU board; designed around the "
                a {
                  class: "link",
                  href: "https://github.com/circuitdojo/nrf9160-feather",
                  "Circuit Dojo nRF9160 Feather"
                }
                "."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/neopixel-6-display-controller",
                Icon { icon: LdGithub }
                "GitHub"
              }
            }
          }
        }

        div { class: "card lg:card-side lg:flex-row-reverse bg-base-200 my-6",
          figure { class: "lg:w-1/2 lg:rounded-r-lg! lg:rounded-l-none!",
            img {
              src: asset!("assets/images/neopixel-seven-segment-display.png"),
              alt: "Neopixel three-digit seven-segment display circuit board render",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h3 { class: "text-2xl card-title", "Neopixel Seven Segment Display" }
              p { class: "text-lg font-medium mt-2",
                "A big, daisy-chainable, three-digit display."
              }
              Tags { tags: vec!["WS2813B-V5", "3.8–32V input"] }
              ul { class: "list-outside list-disc pl-8 my-2",
                li {
                  "63 "
                  a {
                    class: "link",
                    href: "https://media.digikey.com/pdf/Data%20Sheets/Seeed%20Technology/WS2813B_Ver.V5_10-20-19.pdf",
                    "WS2813B-V5"
                  }
                  " LED pixels — three per segment"
                }
                li { "2A, 5V synchronous buck converter with a 3.8V to 32V input range" }
                li { "Spring-clamp connectors and backup data-signal jumpers for chaining" }
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/neopixel-seven-segment-display",
                Icon { icon: LdGithub }
                "GitHub"
              }
            }
          }
        }
      }
    }
  }
}
