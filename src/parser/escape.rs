use crate::core::re;
use crate::core::escape;
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: HTML escapes
  let re_escape: Regex = re::from(re::ESCAPE);
  re::parse(html, re_escape, | capture: Captures |
    escape::ascii(&capture[1])
  );
}
