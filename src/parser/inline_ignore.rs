use crate::core::{id, re};
use regex::{Captures, Regex};
use std::collections::HashMap;

pub fn hide(html: &mut String) -> HashMap<i32, String> {
  // Store ignored scripts & styles
  let mut ignore: HashMap<i32, String> = HashMap::new();

  // Parse: Inline ignore script
  let re_inline_script: Regex = re::from(re::INLINE_SCRIPT);
  re::parse(html, re_inline_script, | capture: Captures | {
    // Create a unique ID
    let mut id: i32 = id::random_10_digit();
    while ignore.contains_key(&id) {
      id = id::random_10_digit();
    }
    // 1. Create the text ID for the ignored script and nsert it into the ignore hashmap
    let text_id: String = format!("?{}?", &id);
    ignore.insert(id, capture[0].to_string());
    // Return the text ID string as the temporary value
    text_id
  });

  // Parse: Inline ignore style
  let re_inline_style: Regex = re::from(re::INLINE_STYLE);
  re::parse(html, re_inline_style, | capture: Captures | {
    // Create a unique ID
    let mut id: i32 = id::random_10_digit();
    while ignore.contains_key(&id) {
      id = id::random_10_digit();
    }
    // 1. Create the text ID for the ignored style and nsert it into the ignore hashmap
    let text_id: String = format!("?{}?", &id);
    ignore.insert(id, capture[0].to_string());
    // Return the text ID string as the temporary value
    text_id
  });

  // Return a hashmap containing the ignored scripts & styles
  ignore
}

pub fn show(html: &mut String, ignore: HashMap<i32, String>) {
  for (id, original) in ignore {
    *html = html.replace(&format!("?{}?", id), &original);
  }
}
