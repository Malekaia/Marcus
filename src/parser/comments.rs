use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String, allow_comments: bool) {
  // Create the regex
  let re: Regex = Regex::new(RE::COMMENTS).unwrap();
  // Check for comments
  if re.is_match(html) && allow_comments == false {
    // Iter captures
    for capture in re.captures_iter(html.clone().as_ref()) {
      // Remove the comments if requested
      *html = html.replace(&capture[0], "");
    }
  }
}
