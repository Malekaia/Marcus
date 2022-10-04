use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

fn is_rule(line: &String, value: &str) -> bool {
  line.starts_with(value) && line.matches(value).count() >= 3 && line.replace(value, "").len() == 0
}

pub fn default(html: &mut String) {
  replacer(html, RE::HORIZONTAL_RULE, | capture: Captures | {
    // Trim the line and remove whitespace
    let line: &String = &capture[0].trim().replace(" ", "");
    // Return a horizontal rule or the capture as was (if not a horizontal rule)
    if line.len() >= 3 && (is_rule(line, "*") || is_rule(line, "-") || is_rule(line, "_")) {
      String::from("\n\n<hr />\n\n")
    } else {
      capture[0].to_string()
    }
  });
}
