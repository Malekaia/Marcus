use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  replacer(html, RE::SUPERSCRIPT, | capture: Captures |
    format!("<sup>{}</sup>", &capture[1].trim())
  );
}
