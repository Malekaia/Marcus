use crate::core::re;
use regex::Captures;

// Parse: Strikethroughs
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::STRIKETHROUGH), | capture: Captures |
    format!("<del>{}</del>", &capture[1].trim())
  );
}
