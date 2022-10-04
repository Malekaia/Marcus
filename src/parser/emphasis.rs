use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

// Get the capture index for the emphasis ("*" = 1, "_" = 2)
fn capture_index(value: &str) -> usize {
  if value.starts_with("*") {
    1
  } else if value.starts_with("_") {
    2
  } else {
    panic!("Marcus: ParseError: Emphasis match doesn't start with \"*\" or \"_\".")
  }
}

// Ignore emphasis matches with no text
fn capture_or_html(text: &str, capture: &str, html: String) -> String {
  if text.len() < 1 {
    capture.to_string()
  } else {
    html
  }
}

pub fn default(html: &mut String) {
  // Bold Italic Emphasis
  replacer(html, RE::EMPHASIS_BOLD_ITALIC, | capture: Captures | {
    let text: &str = capture[capture_index(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<b><i>{}</i></b>", text))
  });

  // Bold Emphasis
  replacer(html, RE::EMPHASIS_BOLD, | capture: Captures | {
    let text: &str = capture[capture_index(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<b>{}</b>", text))
  });

  // Italic Emphasis
  replacer(html, RE::EMPHASIS_ITALIC, | capture: Captures | {
    let text: &str = capture[capture_index(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<i>{}</i>", text))
  });
}
