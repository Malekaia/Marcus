use crate::core::re;
use regex::Regex;

// Parse: Explicit paragraphs
pub fn default(html: &mut String) {
  let re_paragraph: Regex = re::from(re::PARAGRAPH);
  let re_inline_ignore: Regex = re::from(r"\?[0-9]{10}\?");
  *html = html.lines()
    .map(| line: &str | -> String {
      let trimmed: &str = line.trim();
      if re_paragraph.is_match(trimmed) && !re_inline_ignore.is_match(trimmed) {
        // Replace all leading whitespace with a single space
        let prefix: &str = if line.starts_with(" ") { " " } else { "" };
        return format!("<p>{prefix}{trimmed}</p>");
      } else {
        line.to_string()
      }
    })
    .fold(String::new(),
      | a: String, b: String | format!("{a}\n{b}")
    );
}
