use crate::core::re;
use regex::{Captures, Regex};

// Determine if a line is a horizontal rule
fn is_rule(line: &String, value: &str) -> bool {
  line.starts_with(value) && line.matches(value).count() >= 3 && line.replace(value, "").is_empty()
}

pub fn default(html: &mut String) {
  // Parse: Horizontal Rules
  let re_horizontal_rule: Regex = re::from(re::HORIZONTAL_RULE);
  re::parse(html, re_horizontal_rule, | capture: Captures | {
    // Trim the line and remove whitespace
    let line: &String = &capture[0].trim().replace(" ", "");
    // Return a horizontal rule or the capture as was (if not a horizontal rule)
    let has_rule: bool = is_rule(line, "*") || is_rule(line, "-") || is_rule(line, "_");
    (if line.len() >= 3 && has_rule { "\n\n<hr />\n\n" } else { &capture[0] }).to_string()
  });
}
