use crate::helpers::re;
use regex::Captures;

// Parse: Comments
pub fn default(html: &mut String) {
  re::parse(html, re::from(re::COMMENTS), | _: Captures | String::new());
}
