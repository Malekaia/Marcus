use crate::core::escape;
use crate::core::re;
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: Auto links
  let regex: Regex = re::from(re::AUTO_LINK);
  re::parse(html, regex, | capture: Captures | {
    format!("<a href=\"{link}\">{link}</a>", link = &capture[1])
  });

  // Parse: Auto emails
  // Email Validation Source: https://emailregex.com/
  let regex: Regex = re::from(re::AUTO_LINK_EMAIL);
  re::parse(html, regex, | capture: Captures | {
    format!("<a href=\"mailto:{email}\">{email}</a>", email = escape::ascii(&capture[1]))
  });
}
