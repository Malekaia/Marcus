use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  // Headings (with ID), extract the heading's size, text and id
  replacer(html, RE::HEADING_WITH_ID, | capture: Captures |
    format!(
      "<h{size} id=\"{id}\">{text}</h{size}>\n",
      size = capture[1].trim().len(), text = capture[2].trim(), id = capture[3].trim()
    )
  );

  // Headings (without ID), extract the heading's size and text
  replacer(html, RE::HEADING, | capture: Captures |
    format!("<h{size}>{text}</h{size}>\n", size = capture[1].trim().len(), text = capture[2].trim())
  );
}
