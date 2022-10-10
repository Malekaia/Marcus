use crate::core::re;
use regex::{Captures, Regex};

// Parse: Optional Comments
pub fn default(html: &mut String, allow_comments: bool) {
  let re_comments: Regex = re::from(re::COMMENTS);
  re::parse(html, re_comments, | capture: Captures |
    if allow_comments == false { String::new() } else { capture[0].to_string() }
  );
}
