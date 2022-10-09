use crate::core::re;
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: Fenced code blocks
  let re_fenced_code_block: Regex = re::from(re::FENCED_CODE_BLOCK);
  re::parse(html, re_fenced_code_block, | capture: Captures | {
    // Get the language type and source code
    let (language, code): (&str, &str) = (capture[1].trim(), capture[2].trim());
    // Return the parsed HTML
    format!("<pre{}>\n{code}\n</pre>", if language.len() > 0 { format!(" lang=\"{language}\"") } else { String::new() })
  });

  // Parse: Inline code
  let re_inline_code: Regex = re::from(re::INLINE_CODE);
  re::parse(html, re_inline_code, | capture: Captures |
    format!("<code>{}</code>", &capture[1].trim())
  );
}
