use crate::core::re;
use regex::{Captures, Regex};

// Parse: Strikethroughs
pub fn default(html: &mut String) {
  let re_strikethrough: Regex = re::from(re::STRIKETHROUGH);
  re::parse(html, re_strikethrough, | capture: Captures |
    format!("<del>{}</del>", &capture[1].trim())
  );
}
