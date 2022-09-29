use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  // Inline code
  replacer(html, RE::INLINE_CODE, | capture: Captures |
    format!("<code>{}</code>", &capture[1].trim())
  );
}
