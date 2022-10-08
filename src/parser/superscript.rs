use crate::core::re;
use regex::{Captures, Regex};

// Parse: Subscript
pub fn default(html: &mut String) {
  let re_superscript: Regex = re::from(re::SUPERSCRIPT);
  re::parse(html, re_superscript, | capture: Captures |
    format!("<sup>{}</sup>", &capture[1].trim())
  );
}
