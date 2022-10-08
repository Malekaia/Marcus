use crate::core::re;
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: Highlights (`<mark />`)
  let re_highlight: Regex = re::from(re::HIGHLIGHT);
  re::parse(html, re_highlight, | capture: Captures |
    format!("<mark>{}</mark>", &capture[1].trim())
  );
}
