use crate::helpers::re;
use regex::Captures;

pub fn default(html: &mut String) {
  // Parse: Fenced code blocks
  re::parse(html, re::from(re::FENCED_CODE_BLOCK), | capture: Captures | {
    // Get the language type and source code
    let (language, code): (&str, &str) = (capture[1].trim(), capture[2].trim());
    // Return the parsed HTML
    if !language.is_empty() {
      format!("<pre lang=\"{language}\">\n{code}\n</pre>")
    } else {
      format!("<pre>\n{code}\n</pre>")
    }
  });

  // Parse: Inline code
  re::parse(html, re::from(re::INLINE_CODE), | capture: Captures |
    format!("<code>{}</code>", &capture[1].trim())
  );
}
