use crate::helpers::re;
use regex::Captures;

// Get the capture index for the emphasis ('*' = 1, '_' = 2, otherwise panic)
fn index_of_capture(value: &str) -> usize {
  match value.chars().nth(0).unwrap() {
    '*' => 1,
    '_' => 2,
     _  => panic!("Marcus: ParseError: Emphasis match doesn't start with \"*\" or \"_\".")
  }
}

// Ignore emphasis matches with no text
fn capture_or_html(text: &str, capture: &str, html: String) -> String {
  if text.len() < 1 { capture.to_string() } else { html }
}

pub fn default(html: &mut String) {
  // Parse: Bold Italic Emphasis
  re::parse(html, re::from(re::EMPHASIS_BOLD_ITALIC), | capture: Captures | {
    let text: &str = capture[index_of_capture(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<b><i>{}</i></b>", text))
  });

  // Parse: Bold Emphasis
  re::parse(html, re::from(re::EMPHASIS_BOLD), | capture: Captures | {
    let text: &str = capture[index_of_capture(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<b>{}</b>", text))
  });

  // Parse: Italic Emphasis
  re::parse(html, re::from(re::EMPHASIS_ITALIC), | capture: Captures | {
    let text: &str = capture[index_of_capture(&capture[0])].trim();
    capture_or_html(&text, &capture[0], format!("<i>{}</i>", text))
  });
}
