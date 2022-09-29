use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String, allow_comments: bool) {
  // Optionally remove comments
  replacer(html, RE::COMMENTS, | capture: Captures |
    if allow_comments == false { String::new() } else { capture[0].to_string() }
  );
}
