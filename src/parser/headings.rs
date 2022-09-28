use crate::regex::RE;
use crate::parser::replacer;

pub fn default(html: &mut String) {
  // Headings (with ID)
  replacer(html, RE::HEADING_WITH_ID, | capture | {
    // Extract the heading's size, text and id
    let (size, text, id): (usize, &str, &str) = (capture[1].trim().len(), capture[2].trim(), capture[3].trim());
    (capture[0].to_string(), format!("<h{size} id=\"{id}\">{text}</h{size}>\n"))
  });

  // Headings (without ID)
  replacer(html, RE::HEADING, | capture | {
    // Extract the heading's size and text
    let (size, text): (usize, &str) = (capture[1].trim().len(), capture[2].trim());
    (capture[0].to_string(), format!("<h{size}>{text}</h{size}>\n"))
  });
}
