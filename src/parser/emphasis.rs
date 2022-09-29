use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

fn index(value: &str) -> usize {
  if value.starts_with("*") { 1 } else {  2 }
}

pub fn default(html: &mut String) {
  // Bold Italic Emphasis
  replacer(html, RE::EMPHASIS_BOLD_ITALIC, | capture: Captures |
    format!("<b><i>{}</i></b>", &capture[index(&capture[0])].trim())
  );

  // Bold Emphasis
  replacer(html, RE::EMPHASIS_BOLD, | capture: Captures |
    format!("<b>{}</b>", &capture[index(&capture[0])].trim())
  );

  // Italic Emphasis
  replacer(html, RE::EMPHASIS_ITALIC, | capture: Captures |
    format!("<i>{}</i>", &capture[index(&capture[0])].trim())
  );
}
