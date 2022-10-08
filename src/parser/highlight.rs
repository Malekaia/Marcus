use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  replacer(html, RE::HIGHLIGHT, | capture: Captures |
    format!("<mark>{}</mark>", &capture[1].trim())
  );
}
