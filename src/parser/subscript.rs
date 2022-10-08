use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  replacer(html, RE::SUBSCRIPT, | capture: Captures |
    format!("<sub>{}</sub>", &capture[1].trim())
  );
}
