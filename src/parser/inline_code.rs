use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String) {
  // Create the regex
  let re: Regex = Regex::new(RE::INLINE_CODE).unwrap();
  // Check for inline code
  if re.is_match(html) {
    // Iter all captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      // Replace the inline code
      *html = html.replace(&capture[0], &format!("<code>{}</code>", &capture[1]));
    }
  }
}
