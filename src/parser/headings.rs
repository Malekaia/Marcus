use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String) {
  /*
   * HEADINGS (WITH ID)
   */
  // Create the regex
  let re: Regex = Regex::new(RE::HEADING_WITH_ID).unwrap();
  // Check for headings with ID
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      unimplemented!();
    }
  }

  /*
   * HEADINGS (WITHOUT ID)
   */
  // Create the regex
  let re: Regex = Regex::new(RE::HEADING).unwrap();
  // Check for headings without ID
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      unimplemented!();
    }
  }
}
