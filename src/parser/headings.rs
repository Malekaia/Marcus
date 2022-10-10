use crate::core::re;
use regex::Captures;

pub fn default(html: &mut String) {
  // Parse: Headings (with ID)
  re::parse(html, re::from(re::HEADING_WITH_ID), | capture: Captures | {
    // Extract the heading's size, text and id
    let (prefix, size, text, id): (&str, usize, &str, &str) = (&capture[1], capture[2].trim().len(), capture[3].trim(), capture[4].trim());
    // Return the formatted text
    format!("{prefix}<h{size} id=\"{id}\">{text}</h{size}>\n")
  });

  // Parse: Headings (without ID)
  re::parse(html, re::from(re::HEADING), | capture: Captures | {
    // Extract the heading's size and text
    let (prefix, size, text): (&str, usize, &str) = (&capture[1], capture[2].trim().len(), capture[3].trim());
    // Return the formatted text
    format!("{prefix}<h{size}>{text}</h{size}>\n")
  });
}
