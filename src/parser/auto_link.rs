use crate::core::{escape, re};
use regex::Captures;

pub fn default(html: &mut String) {
  // Parse: Auto links
  re::parse(html, re::from(re::AUTO_LINK), | capture: Captures |
    format!("<a href=\"{link}\">{link}</a>", link = &capture[1])
  );

  // Parse: Auto emails (email validation regex source: https://emailregex.com/)
  re::parse(html, re::from(re::AUTO_LINK_EMAIL), | capture: Captures |
    format!("<a href=\"mailto:{email}\">{email}</a>", email = escape::ascii(&capture[1]))
  );
}
