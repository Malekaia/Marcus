use crate::helpers::re;
use regex::Regex;

// Parse: Explicit paragraphs
pub fn default(html: &mut String) {
  let (re_paragraph, re_inline_ignore): (Regex, Regex) = (re::from(re::PARAGRAPH), re::from(r"\?[0-9]{10}\?"));
  // Cleanup the HTML and map it's lines
  let output: String = html.trim().lines().map(| line: &str | -> String {
    // Create a temporary trimmed line
    let trimmed: &str = line.trim();
    // Create the paragraph if it matches & replace all leading whitespace with a single space
    if re_paragraph.is_match(trimmed) && !re_inline_ignore.is_match(trimmed) {
      format!("<p>{prefix}{trimmed}</p>", prefix = if line.starts_with(" ") { " " } else { "" })
    }
    // Return the line as is
    else {
      line.to_string()
    }
  })
  // Join the lines with a new line
  .fold(String::new(), | a: String, b: String | format!("{a}\n{b}"));
  // Replace multiple new lines with a single new line
  *html = re::from(r"\n+").replace(&output.trim(), "\n\n").to_string()
}
