use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdGithub;

#[component]
pub fn Projects() -> Element {
  rsx! {
    div { class: "flex-1 py-8 lg:py-12 px-2 lg:px-8",

      section { id: "software",
        h1 { class: "text-4xl font-bold", "Software" }
        div { class: "divider mt-0" }

        div { class: "card lg:card-side bg-base-200 my-6",
          figure { class: "lg:w-1/2",
            img {
              src: asset!("assets/images/departure-sign.jpg"),
              alt: "Departure Board",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h2 { class: "card-title mb-4", "Real-time Cellular Departure Board" }
              p { class: "my-4",
                "A Zephyr based firmware for a real-time, cellular connected, solar-powered
                departure board. Writen for a Nordic nRF9160 based system."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-end",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/embedded-departure-board",
                Icon { icon: LdGithub }
              }
            }
          }
        }
      }

      section { id: "hardware",
        h1 { class: "text-4xl font-bold", "Hardware" }
        div { class: "divider mt-0" }

        div { class: "card lg:card-side bg-base-200 my-6",
          figure { class: "lg:w-1/2",
            img {
              src: asset!("assets/images/neopixel-6-display-controller.png"),
              alt: "neopixel 6 display controller",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h2 { class: "card-title mb-4", "Neopixel 6 Display Controller" }
              p { class: "my-4",
                "A control board for up to 6 NZR (neopixel/WS28*) displays.
                This control board was designed to be used with a "
                a {
                  class: "link",
                  href: "https://github.com/circuitdojo/nrf9160-feather",
                  "Circuit Dojo - nRF9160 Feather"
                }
                ", but can be used with any "
                a {
                  class: "link",
                  href: "https://learn.adafruit.com/adafruit-feather/feather-specification",
                  "Adafruit Feather compatible"
                }
                " MCU board."
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-end",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/neopixel-6-display-controller",
                Icon { icon: LdGithub }
              }
            }
          }
        }

        div { class: "card lg:card-side lg:flex-row-reverse bg-base-200 my-6",
          figure { class: "lg:w-1/2",
            img {
              src: asset!("assets/images/neopixel-seven-segment-display.png"),
              alt: "neopixel seven segment display",
            }
          }
          div { class: "card-body lg:w-1/2",
            article { class: "flex-1",
              h2 { class: "card-title mb-4", "Neopixel 6 Display Controller" }
              p { class: "my-4",
                "A large, daisy chain-able, 3 digit seven-segment display board. Constructed with "
                a {
                  class: "link",
                  href: "https://media.digikey.com/pdf/Data%20Sheets/Seeed%20Technology/WS2813B_Ver.V5_10-20-19.pdf",
                  "WS2813B-V5"
                }
                " LED pixels and a 5V fixed output, synchronous buck converter."
              }
              p { class: "my-4",
                h3 { "Features:" }
                ul { class: "list-outside list-disc pl-8",
                  li { "63 total WS2813B-V5 LED pixels" }
                  li { "3 WS2813B-V5 LED pixels per segment" }
                  li { "2 6-pin spring clamp connectors" }
                  li {
                    "2A, 5V fixed output, synchronous buck converter with a wide input voltage range of 3.8V to 32V"
                  }
                  li { "Backup NZR data signal to ground jumper pins" }
                }
              }
            }
            div { class: "card-actions justify-center-safe lg:justify-start",
              a {
                class: "btn btn-soft",
                href: "https://github.com/umts/neopixel-seven-segment-display",
                Icon { icon: LdGithub }
              }
            }
          }
        }
      }
    }
  }
}
