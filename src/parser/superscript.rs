use crate::core::re;
use regex::Captures;

// Parse: Subscript
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::SUPERSCRIPT), | capture: Captures |
    format!("<sup>{}</sup>", &capture[1].trim())
  );
}
