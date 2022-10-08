use regex::{Captures, Regex};

pub fn from(regex: &str) -> Regex {
  Regex::new(regex).unwrap()
}

pub fn parse<F: FnMut(Captures) -> String>(html: &mut String, regex: Regex, mut parser: F) {
  // Ignore non-matches
  if regex.is_match(&html) {
    // Insert full capture and parsed HTML into the result vector
    let mut result: Vec<(String, String)> = vec![];
    for capture in regex.captures_iter(&html) {
      result.push((capture[0].to_string(), parser(capture)));
    }
    // Replace the MD capture with the parsed HTML
    for (capture, parsed) in result {
      *html = html.replace(&capture, &parsed);
    }
  }
}

/* BASIC */
// Order: first (above all else)
pub const ESCAPE: &str = r"\\(.{1})";
// Optional
pub const COMMENTS: &str = r"<!--(.*?)-->";
pub const BLOCKQUOTE: &str = r"(^|\n+)([>]{1,}.*?(\n)+)+";
pub const INLINE_CODE: &str = r"`{1}([^`]+)`{1}";
// Order: before bold emphasis
pub const EMPHASIS_BOLD_ITALIC: &str = r"[*]{3}(.*?)[*]{3}|[_]{3}(.*?)[_]{3}";
// Order: before italic emphasis
pub const EMPHASIS_BOLD: &str = r"[*]{2}(.*?)[*]{2}|[_]{2}(.*?)[_]{2}";
pub const EMPHASIS_ITALIC: &str = r"[*]{1}(.*?)[*]{1}|[_]{1}(.*?)[_]{1}";
pub const HEADING: &str = r"(^|\n+)([#]{1,})(.*?)\n+";
pub const HORIZONTAL_RULE: &str = r"\n[\s]{0,}(\*|\-|_)(.*?)\n";
pub const LINK_DEFINE_WITH_TITLE: &str = r#"[\S]{0,}\[(.*?)\]:(.*?)('|"|\()(.*?)('|"|\))\n"#;
pub const LINK_DEFINE: &str = r"[\S]{0,}\[(.*?)\]:(.*?)\n";
pub const IMAGE_WITH_TITLE: &str = r#"!\[(.*?)\]\((.*?)('|"|\()(.*?)('|"|\))\)"#;
pub const IMAGE: &str = r"!\[(.*?)\]\((.*?)\)";
pub const LINK_WITH_TITLE: &str = r#"\[(.*?)\]\((.*?)('|"|\()(.*?)('|"|\))\)"#;
pub const LINK: &str = r"\[(.*?)\]\((.*?)\)";
pub const LIST_ITEM: &str = r"(^|\n)((\s{0,})([0-9]{1,}\.|\-|\*)(.*?)\n)+";
// Paragraphs are parsed last after everything else
#[allow(unused)]
pub const PARAGRAPH: &str = r"(^|\n+)([A-Za-z0-9].*?)(\n|$)"; // Order: last item
/* CUSTOM */
pub const AUTO_LINK: &str = r"<(http[s]{0,1}://.*?)/>";
// Email Validation Source: https://emailregex.com/
pub const AUTO_LINK_EMAIL: &str = r#"<((?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\]))/>"#;
pub const INLINE_SCRIPT: &str = r"<script.*?>(\n|.)*?</script>";
pub const INLINE_STYLE: &str = r"<style.*?>(\n|.)*?</style>";
/* EXTENDED */
#[allow(unused)]
pub const DEFINITION_LIST: &str = r"(^|\n)([A-Za-z0-9].*?)\n([\s]{0,}:(.*?)\n)+";
pub const EMOJI: &str = r":([a-zA-Z_]+):";
pub const FENCED_CODE_BLOCK: &str = r"```(.*?)\n((\n|.)*?)```";
/* Footnote definitions are automatically caught and parsed by
 * the "LINK_DEFINE_WITH_TITLE" and "LINK_DEFINE" regular expressions
 * pub const FOOTNOTE_DEFINITION: &str = r"\[\^([0-9]{1,})\][\s]{0,}:[\s]{0,}(.*?)(\n|$)";
 */
pub const FOOTNOTE: &str = r"\[\^([0-9]{1,})\]";
// Order: before heading without ID
pub const HEADING_WITH_ID: &str = r"(^|\n+)([#]{1,})(.*?)\{\#(.*?)\}\n+";
pub const HIGHLIGHT: &str = r"==(.*?)==";
pub const STRIKETHROUGH: &str = r"~~(.*?)~~";
pub const SUBSCRIPT: &str = r"~(.*?)~";
pub const SUPERSCRIPT: &str = r"\^(.*?)\^";
#[allow(unused)]
pub const TABLE: &str = r"(\|.*?\|(\n|$))+";
/* Task list checkboxes are parsed with the list item module
 * pub const TASK_LIST_IDENTIFIER: &str = r"^(\[[\s]{0,}[xX]{1}[\s]{0,}\]|\[[\s]{0,}\])\s{0,}";
 */
