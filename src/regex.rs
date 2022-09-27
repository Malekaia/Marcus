#[allow(dead_code, non_snake_case)]
pub mod RE {
  /*
   * BASIC
   */
  pub const ESCAPE: &str = r"\\(.{1})"; // Order: first in order
  pub const COMMENTS: &str = r"<!--(.*?)-->"; // Optional
  pub const BLOCKQUOTE: &str = r"([>]{1,}.*?(\n)+)+";
  pub const INLINE_CODE: &str = r"`{1}([^`]+)`{1}";
  pub const EMPHASIS_BOLD_ITALIC: &str = r"[*]{3}(.*?)[*]{3}|[_]{3}(.*?)[_]{3}"; // Order: before bold emphasis
  pub const EMPHASIS_BOLD: &str = r"[*]{2}(.*?)[*]{2}|[_]{2}(.*?)[_]{2}"; // Order: before italic emphasis
  pub const EMPHASIS_ITALIC: &str = r"[*]{1}(.*?)[*]{1}|[_]{1}(.*?)[_]{1}";
  pub const HEADING: &str = r"(?<=(^|\n))(#+)(.*?)(\n)";
  pub const HORIZONTAL_RULE: &str = r"\n([\s]{0,}[*-_][\s]{0,}){3,}\n";
  pub const IMAGE_WITH_TITLE: &str = r#"\!\[(.*?)\]\((.*?)\((.*?)\)\)|\!\[(.*?)\]\((.*?)\"(.*?)\"\)|\!\[(.*?)\]\((.*?)\'(.*?)\'\)"#;
  pub const IMAGE: &str = r"\!\[(.*?)\]\((.*?)\)";
  pub const LINK_DEFINE_WITH_TITLE: &str = r#"\[.*?\]\:((.*?)(\"(.*?)\"|\'(.*?)\'|\((.*?)\)))\n"#;
  pub const LINK_DEFINE: &str = r"\[.*?\]\:(.*?)\n";
  pub const LINK_WITH_TITLE: &str = r#"\[(.*?)\]\((.*?)\"(.*?)\"\)|\[(.*?)\]\((.*?)\'(.*?)\'\)"#;
  pub const LINK: &str = r"\[(.*?)\]\((.*?)\)";
  pub const LIST_ITEM: &str = r"((^|[ \t]{0,})([0-9]\.|\*|\-)(.*?)\n)+";
  pub const TASK_LIST: &str = r"(^|\n)[\s]{0,}\-[\s]{0,}\[([\s]{1,}|x)\]"; // CATEGORY: CUSTOM
  pub const PARAGRAPH: &str = r"(^|\n+)([A-Za-z0-9].*?)(\n|$)"; // Order: last item

  /*
   * CUSTOM
   */
  pub const AUTOLINK: &str = r"<(http[s]{0,1}::.*?)\/>";
  // Email Validation Source: https://emailregex.com/
  pub const AUTOLINK_EMAIL: &str = r#"<((?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\]))\/>"#;
  pub const INLINE_SCRIPT: &str = r"<script.*?>(\n|.)*?<\/script>";
  pub const INLINE_STYLE: &str = r"<style.*?>(\n|.)*?<\/style>";

  /*
   * EXTENDED
   */
  pub const DEFINITION_LIST: &str = r"(^|\n)([A-Za-z0-9].*?)\n([\s]{0,}:(.*?)\n)+";
  pub const EMOJI: &str = r"([\s]{1,}:[A-Za-z-_]+:[\s]{1,})";
  pub const FENCED_CODE_BLOCK: &str = r"```(\n|.)*?```";
  pub const FOOTNOTE_DEFINITION: &str = r"\[\^([0-9]{1,})\][\s]{0,}:[\s]{0,}(.*?)(\n|$)";
  pub const FOOTNOTE: &str = r"\[\^([0-9]{1,})\]";
  pub const HEADING_WITH_ID: &str = r"(?<=(^|\n))(#+)(.*?){#(.*?)}(\n)"; // Order: before heading without ID, CATEGORY: CUSTOM
  pub const HIGHLIGHT: &str = r"==(.*?)==";
  pub const STRIKETHROUGH: &str = r"~~(.*?)~~";
  pub const SUBSCRIPT: &str = r"~(.*?)~";
  pub const SUPERSCRIPT: &str = r"\^(.*?)\^";
  pub const TABLE: &str = r"(\|.*?\|(\n|$))+";
}
