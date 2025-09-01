use dioxus::prelude::*;
#[cfg(not(feature = "web"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "web")]
use web_time::{SystemTime, UNIX_EPOCH};
#[component]
pub fn Footer() -> Element {
  rsx! {
    footer { class: "footer footer-horizontal footer-center bg-base-200 text-base-content lg:p-2",
      aside {
        p {
          "© {1970 + (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()/31556926)} "
          span { class: "text-nowrap", "Justin's Engineering Services, LLC" }
        }
      }
    }
  }
}
