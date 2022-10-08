use crate::core::re;
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: Headings (with ID)
  let re_heading_with_id: Regex = re::from(re::HEADING_WITH_ID);
  re::parse(html, re_heading_with_id, | capture: Captures |
    format!(
      "{prefix}<h{size} id=\"{id}\">{text}</h{size}>\n",
      // Extract the heading's size, text and id
      prefix = &capture[1], size = capture[2].trim().len(),
      text = capture[3].trim(), id = capture[4].trim()
    )
  );

  // Parse: Headings (without ID)
  let re_heading: Regex = re::from(re::HEADING);
  re::parse(html, re_heading, | capture: Captures |
    format!(
      "{prefix}<h{size}>{text}</h{size}>\n",
      // Extract the heading's size and text
      prefix = &capture[1],
      size = capture[2].trim().len(),
      text = capture[3].trim()
    )
  );
}
