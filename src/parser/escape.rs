use crate::core::{escape, re};
use regex::Captures;

// Parse: HTML escapes
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::ESCAPE), | capture: Captures |
    escape::ascii(&capture[1])
  );
}
