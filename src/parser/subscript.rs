use crate::core::re;
use regex::{Captures, Regex};

// Parse: Subscript
pub fn default(html: &mut String) {
  let re_subscript: Regex = re::from(re::SUBSCRIPT);
  re::parse(html, re_subscript, | capture: Captures |
    format!("<sub>{}</sub>", &capture[1].trim())
  );
}
