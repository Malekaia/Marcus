use crate::core::re;
use regex::{Captures, Regex};

// Parse: Optional Comments
pub fn default(html: &mut String) {
  let re_comments: Regex = re::from(re::COMMENTS);
  re::parse(html, re_comments, | _: Captures |
    String::new()
  );
}
