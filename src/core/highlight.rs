use crate::helpers::re;
use regex::Captures;

// Parse: Highlights (`<mark />`)
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::HIGHLIGHT), | capture: Captures |
    format!("<mark>{}</mark>", &capture[1].trim())
  );
}
