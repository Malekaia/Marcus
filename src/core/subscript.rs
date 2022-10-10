use crate::helpers::re;
use regex::Captures;

// Parse: Subscript
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::SUBSCRIPT), | capture: Captures |
    format!("<sub>{}</sub>", &capture[1].trim())
  );
}
