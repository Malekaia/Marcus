use crate::regex::RE;
use crate::parser::replacer;
use html_escape;
use regex::Captures;

pub fn default(html: &mut String) {
  // HTML encode the escaped character (or return the raw character if not required)
  replacer(html, RE::ESCAPE, | capture: Captures |
    html_escape::encode_safe(&capture[1]).to_string()
  );
}
