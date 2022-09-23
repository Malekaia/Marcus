use crate::regex::RE;
use regex::Regex;

#[derive(Debug)]
pub struct Blockquote {
  pub count: usize,
  pub text: String,
  pub cite: String,
}

pub fn default(html: &mut String) {
  // Create the escape regex
  let re: Regex = Regex::new(RE::BLOCKQUOTE).unwrap();
  // Check for escaped characters
  if re.is_match(html) {
    // Define vectors to group the blockquotes
    let mut blockquote_groups: Vec<Vec<Blockquote>> = vec![];
    let mut current_group: Vec<Blockquote> = vec![];
    // Extract all blockquote groups
    for capture in re.captures_iter(html) {
      // Extract each blockquote in the current group
      for mut quote in capture[0].trim().split("\n").map(| item: &str | item.trim().to_string()).collect::<Vec<String>>() {
        // Define the blockquote properties
        let mut count: usize = 0;
        let (text, cite): (String, String);
        // Determine size of blockquote
        while quote.starts_with(">") {
          quote.remove(0);
          count += 1;
        }
        // Get the blockquote text & trim it
        quote = quote.trim().to_string();
        // Determine if the blockquote contains text and/or cite & extract them
        if quote.contains("--") {
          // Separate blockquote text and cite
          let text_cite: Vec<String> = quote.split(" -- ").map(| pair: &str | pair.trim().to_string()).collect::<Vec<String>>();
          // Blockquote has text but no cite
          if text_cite.len() == 1 {
            (text, cite) = (text_cite[0].to_owned(), String::new());
          // Blockquote has text and cite
          } else if text_cite.len() == 2 {
            (text, cite) = (text_cite[0].to_owned(), text_cite[1].to_owned());
          // Blockquote has text and multiple cite definitions
          } else {
            (text, cite) = (text_cite[0].to_owned(), text_cite[1..].join(", "));
          }
        // Blockquote has no cite
        } else {
          (text, cite) = (quote, String::new());
        }
        // Add the current blockquote to it's group
        current_group.push(Blockquote { count, cite, text });
      }
      // Add the group to the group vector and clear it for the next iteration
      blockquote_groups.push(current_group);
      current_group = vec![];
    }
    // Debug
    println!("{:#?}", blockquote_groups);
  }
}
