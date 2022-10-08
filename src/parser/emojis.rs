use crate::core::{emoji, re};
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: MD Emojis
  let re_emoji: Regex = re::from(re::EMOJI);
  re::parse(html, re_emoji, | capture: Captures | {
    for [text, name] in emoji::LIST {
      if name == &capture[1] {
        return text.to_string();
      }
    }
    String::new()
  });
}
