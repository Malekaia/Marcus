use crate::regex::RE;
use crate::parser::replacer;
use regex::Captures;

pub fn default(html: &mut String) {
  // Fenced code block
  replacer(html, RE::FENCED_CODE_BLOCK, | capture: Captures | {
    // Get the language type and source code
    let (language, code): (&str, &str) = (capture[1].trim(), capture[2].trim());
    // Return the parsed HTML
    format!("<pre{}>\n{code}\n</pre>", if language.len() > 0 { " lang=\"{language}\"" } else { "" })
  });

  // Inline code
  replacer(html, RE::INLINE_CODE, | capture: Captures |
    format!("<code>{}</code>", &capture[1].trim())
  );
}
