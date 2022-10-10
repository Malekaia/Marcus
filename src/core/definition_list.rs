use crate::helpers::re;
use regex::{Captures, Regex};

// Parse: Definition list
pub fn default(html: &mut String) {
  let re_dd_start: Regex = re::from(r"^[\s]{0,}:[\s]{0,}(.*?)$");
  re::parse(html, re::from(re::DEFINITION_LIST), | capture: Captures | {
    let mut result: String = format!("{}<dl>\n", &capture[1]);
    for line in capture[0].lines() {
      if line.trim().is_empty() {
        continue;
      } else if re_dd_start.is_match(line) {
        result.push_str(&format!("  <dd>{}</dd>\n", &re_dd_start.captures(line).unwrap()[1].trim()));
      } else {
        result.push_str(&format!("  <dt>{}</dt>\n", line.trim()));
      }
    }
    result + "</dl>\n\n"
  });
}
