use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  // Headings (with ID), extract the heading's size, text and id
  replacer(html, RE::HEADING_WITH_ID, | capture: Captures |
    format!(
      "{prefix}<h{size} id=\"{id}\">{text}</h{size}>\n",
      prefix = &capture[1],
      size = capture[2].trim().len(),
      text = capture[3].trim(),
      id = capture[4].trim()
    )
  );

  // Headings (without ID), extract the heading's size and text
  replacer(html, RE::HEADING, | capture: Captures |
    format!(
      "{prefix}<h{size}>{text}</h{size}>\n",
      prefix = &capture[1],
      size = capture[2].trim().len(),
      text = capture[3].trim()
    )
  );
}
