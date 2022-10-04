use crate::regex::RE;
use regex::{Captures, Regex};

#[derive(Debug)]
pub struct Blockquote {
  pub size: usize,
  pub text: String,
  pub cite: String
}

pub fn default(html: &mut String) {
  // Create the regular expression
  let re: Regex = Regex::new(RE::BLOCKQUOTE).unwrap();

  // Ignore non-matches
  if !re.is_match(&html) {
    return ();
  }

  // Extract and parse MD blockquotes with HTML
  re.captures_iter(&html.clone()).for_each(| capture: Captures | {
    // new HTML string, Blockquote HTML template, previous size
    let (mut prev_size, mut result): (usize, String) = (1, String::new());

    // Convert MD blockquotes to data struct
    capture[0].trim().lines().map(| mut line: &str | -> Blockquote {
      // Find the blockquote size
      let mut size: usize = 0;
      while line.starts_with(">") {
        line = &line[1..];
        size += 1;
      }
      // Remove leading and trailing whitespace
      line = line.trim();
      // Extract blockquote cite
      let (mut cite, cite_split): (&str, &str) = ("", " -- ");
      if line.contains(cite_split) {
        match line.find(cite_split) {
          // Extract the cite and update the quote text
          Some(position) => {
            cite = &line[(position + cite_split.len())..].trim();
            line = &line[..position].trim();
          },
          // ignore non matches
          None => { }
        };
      }
      // Add the blockquote to the current group
      Blockquote {
        // Add the blockquote size and text
        size, text: line.to_string(),
        // Replace multiple cites with commas
        cite: cite.replace(" -- ", ", ")
      }
    })

    // Convert blockquote structs to HTML
    .enumerate().for_each(| (i, blockquote): (usize, Blockquote) | {
      // Get the cite text to replace and it's replacement string
      let (cite_capture, cite_replace): (&str, &str) = if blockquote.cite.len() == 0 {
        ("<cite>%cite</cite>", "")
      } else {
        ("%cite", &blockquote.cite)
      };
        // Create the blockquote HTML and insert the blockquote text
      let blockquote_html: String = format!("<blockquote>{}<cite>%cite</cite>%child</blockquote>%sibling", &blockquote.text)
                                           .replace(cite_capture, cite_replace);
      // First blockquote `size` must be `1`
      if i == 0 && blockquote.size > 1 {
        panic!("Marcus: ParseError: The size (indent level) of the first/root/parent blockquote must be 1");
      // Parent
      } else if i == 0 {
        result = blockquote_html.replace("%sibling", "");
      // Next Sibling of Child
      } else if blockquote.size == prev_size {
        result = result.replace("%sibling", &blockquote_html);
      // Child of Parent
      } else if blockquote.size > prev_size {
        result = result.replace("%child", &blockquote_html);
      // Next Sibling of Parent
      } else {
        result = format!("{}\n\n{}", result.replace("%child", "").replace("%sibling", ""), &blockquote_html);
      }
      // Update the previous size
      prev_size = blockquote.size;
    });
    // Remove unresolved children / siblings, then return the result
    result = format!("\n\n{}\n\n", result.replace("%child", "").replace("%sibling", ""));

    // Replace parsed HTML in HTML string
    *html = html.replace(&capture[0], &result);
  });
}
