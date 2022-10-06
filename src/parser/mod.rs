pub mod escape;
pub mod comments;
pub mod blockquotes;
pub mod inline_code;
pub mod emphasis;
pub mod headings;
pub mod horizontal_rule;
pub mod links_images_footnotes;

use regex::{Captures, Regex};

pub fn replacer<F: FnMut(Captures) -> String>(html: &mut String, regex: &str, mut handler: F) {
  // Create the specified regular expression
  let re: Regex = Regex::new(regex).unwrap();

  // Ignore non-matches
  if !re.is_match(&html) {
    return ();
  }

  // Parse MD to HTML using user defined handler
  let parsed: Vec<(String, String)> = re.captures_iter(&html)
    // Replace captures with return of handler function
    .map(| capture: Captures | (capture[0].to_string(), handler(capture)))
    // collect into vector of string tuple
    .collect::<Vec<(String, String)>>();

  // Replace the MD capture with the parsed HTML
  for (capture, parsed) in parsed {
    *html = html.replace(&capture, &parsed);
  }
}
