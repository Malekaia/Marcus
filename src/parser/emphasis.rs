use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String) {
  /*
   * BOLD ITALIC
   */
  // Create the regex
  let re: Regex = Regex::new(RE::EMPHASIS_BOLD_ITALIC).unwrap();
  // Check for emphasis declarations
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      let index: usize = if capture[0].starts_with("*") { 1 } else {  2 };
      *html = html.replace(&capture[0], &format!("<b><i>{}</i></b>", &capture[index].trim()));
    }
  }

  /*
   * BOLD
   */
  // Create the regex
  let re: Regex = Regex::new(RE::EMPHASIS_BOLD).unwrap();
  // Check for emphasis declarations
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      let index: usize = if capture[0].starts_with("*") { 1 } else {  2 };
      *html = html.replace(&capture[0], &format!("<b>{}</b>", &capture[index].trim()));
    }
  }

  /*
   * BOLD ITALIC
   */
  // Create the regex
  let re: Regex = Regex::new(RE::EMPHASIS_ITALIC).unwrap();
  // Check for emphasis declarations
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      let index: usize = if capture[0].starts_with("*") { 1 } else {  2 };
      *html = html.replace(&capture[0], &format!("<i>{}</i>", &capture[index].trim()));
    }
  }
}
