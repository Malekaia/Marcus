use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  replacer(html, RE::STRIKETHROUGH, | capture: Captures |
    format!("<del>{}</del>", &capture[1].trim())
  );
}
