use crate::core::re;
use regex::Regex;

// Clean up the output HTML
pub fn output(html: &mut String) {
  let re_trimmer: Regex = re::from(r"\n+");
  *html = re_trimmer.replace(html.trim(), "\n\n").to_string()
}
