use crate::regex::RE;
use regex::Regex;

// Data struct for blockquotes
#[derive(Debug)]
pub struct Blockquote {
  pub size: usize,
  pub text: String,
  pub cite: String,
}

pub fn default(html: &mut String) -> () {
  // Create the regex
  let re: Regex = Regex::new(RE::BLOCKQUOTE).unwrap();
  // Items to replace
  let mut blockquote_replacements: Vec<(String, String)> = vec![];
  // Ignore non matches
  if !re.is_match(html) {
    return ();
  }

  // Extract all blockquote groups
  for capture in re.captures_iter(html) {
    /*
     * EXTRACT BLOCKQUOTES
     */
    // Define vectors to group the blockquotes
    let mut blockquote_group: Vec<Blockquote> = vec![];
    // Iterate the captured blockquotes
    for mut text in capture[0].trim().split("\n") {
      // Find the blockquote size
      let mut size: usize = 0;
      while text.starts_with(">") {
        text = &text[1..];
        size += 1;
      }
      // Remove leading and trailing whitespace
      text = text.trim();
      // Extract blockquote cite
      let mut cite: &str = "";
      if text.contains(" -- ") {
        match text.find(" -- ") {
          // Extract the cite and update the quote text
          Some(position) => {
            cite = &text[(position + 4)..].trim();
            text = &text[..position].trim();
          },
          // ignore non matches
          None => { }
        };
      }
      // Add the blockquote to the current group
      blockquote_group.push(Blockquote {
        // Add the blockquote size and text
        size, text: text.to_string(),
        // Replace multiple cites with commas
        cite: cite.replace(" -- ", ", ")
      });
    }

    /*
     * CONVERT TO HTML
     */
    // The new HTML string
    let mut result: String = String::new();
    // Blockquote HTML template
    let blockquote_template: String = "<blockquote>%text\n<cite>%cite</cite>\n%child</blockquote>".to_string();
    // Store the previous size
    let mut prev_size: usize = 1;
    // Iterate the blockquotes
    for (i, blockquote) in blockquote_group.into_iter().enumerate() {
      // Capture the cite and it's replacement
      let (cite_capture, cite_replace): (&str, &str) = if blockquote.cite.len() == 0 {
        ("<cite>%cite</cite>", "")
      } else {
        ("%cite", &blockquote.cite)
      };
      // First blockquote `size` must be `1`
      if i == 0 && blockquote.size > 1 {
        panic!("Marcus: ParseError: Parent blockquote size must be 1");
      }
      // Parent
      else if i == 0 {
        result = blockquote_template.replace("%text", &blockquote.text).replace(cite_capture, cite_replace);
      }
      // Child of Parent
      else if blockquote.size > prev_size {
        result = result.replace("%child", &blockquote_template.replace("%text", &blockquote.text).replace(cite_capture, cite_replace));
      }
      // Next Sibling of Parent
      else {
        result = result.replace("%child", "");
        result.push_str("\n");
        result.push_str(&blockquote_template.replace("%text", &blockquote.text).replace(cite_capture, cite_replace))
      }
      // Update the previous size
      prev_size = blockquote.size;
    }

    /*
     * WRAP UP
     */
    // Remove unresolved children
    result = result.replace("%child", "");
    // Add to replacement vector
    blockquote_replacements.push((capture[0].to_string(), result));
  }

  /*
   * REPLACE MD WITH HTML
   */
  for (capture, replacement) in blockquote_replacements {
    *html = html.replace(&capture, &replacement);
  }
}
