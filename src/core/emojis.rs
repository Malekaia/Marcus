use crate::helpers::{emoji, re};
use regex::Captures;

// Parse: MD Emojis
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::EMOJI), | capture: Captures | {
    for [text, name] in emoji::LIST {
      if name == &capture[1] {
        return text.to_string();
      }
    }
    String::new()
  });
}
