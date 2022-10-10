use crate::core::re;
use regex::Captures;

// Determine if a line is a horizontal rule
fn is_rule(line: &String, value: &str) -> bool {
  line.starts_with(value) && line.matches(value).count() >= 3 && line.replace(value, "").is_empty()
}

// Parse: Horizontal Rules
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::HORIZONTAL_RULE), | capture: Captures | {
    // Trim the line and remove whitespace
    let line: &String = &capture[0].trim().replace(" ", "");
    // Return a horizontal rule or the capture as was (if not a horizontal rule)
    if line.len() >= 3 && (is_rule(line, "*") || is_rule(line, "-") || is_rule(line, "_")) {
      String::from("\n\n<hr />\n\n")
    } else {
      String::from(&capture[0])
    }
  });
}
