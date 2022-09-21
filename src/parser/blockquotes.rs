use crate::regex::RE;
use regex::Regex;

pub fn default(html: &mut String) {
  // Create the escape regex
  let re: Regex = Regex::new(RE::BLOCKQUOTE).unwrap();
  // Check for escaped characters
  if re.is_match(html) {
    for capture in re.captures_iter(html) {
      let blockquote: &str = &capture[0];
      let quotes: Vec<String> = blockquote.trim().split("\n").map(| line: &str | line.trim().to_string()).collect::<Vec<String>>();
      println!("{quotes:?}");
    }
  }
}
