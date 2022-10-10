use crate::core::{escape, re};
use regex::{Captures, Regex};

pub fn default(html: &mut String) {
  // Parse: Auto links
  let re_auto_link: Regex = re::from(re::AUTO_LINK);
  re::parse(html, re_auto_link, | capture: Captures | {
    format!("<a href=\"{link}\">{link}</a>", link = &capture[1])
  });

  // Parse: Auto emails
  // Email Validation Source: https://emailregex.com/
  let re_auto_link_email: Regex = re::from(re::AUTO_LINK_EMAIL);
  re::parse(html, re_auto_link_email, | capture: Captures | {
    format!("<a href=\"mailto:{email}\">{email}</a>", email = escape::ascii(&capture[1]))
  });
}
