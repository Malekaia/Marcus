use crate::core::re;
use regex::{Captures, Regex};
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct Link {
  pub href: String,
  pub title: String,
  pub is_footnote: bool
}

// Prevent link titles from being defined improperly
fn validate_title_definitions(opener: &str, closer: &str, name: &str) {
  if opener != closer && !(opener == "(" && closer == ")") {
    panic!("Marcus: ParseError: The title declaration for {name} must use quotes (\"), apostrophes (') or matching parenthesis \"(\" & \")\"");
  }
}

// Determine if a link's identifier defines a footnote
fn get_footnote(mut identifier: String) -> (String, bool) {
  // Determine if the link is a footnote
  let mut is_footnote: bool = false;
  while identifier.starts_with("^") {
    // Remove the footnote symbol
    identifier = identifier[1..].trim().to_string();
    // Specify that this link is a footnote
    if is_footnote == false {
      is_footnote = true;
    }
  }
  // Return the new identifier and boolean
  (identifier, is_footnote)
}

// Return a string of HTML using a predefined link or a default template
fn html_from_link<F: Fn(&Link) -> String>(links: &HashMap<String, Link>, src: String, defined: String, undefined: F) -> String {
  if links.contains_key(&src) {
    let link: &Link = links.get(&src).unwrap();
    if link.is_footnote == false {
      return undefined(link);
    }
  }
  return defined;
}

pub fn default(html: &mut String) {
  // Store pre defined links in a hashmap:
  let mut links: HashMap<String, Link> = HashMap::new();

  // Link definition (with title):
  let re_link_define_with_title: Regex = re::from(re::LINK_DEFINE_WITH_TITLE);
  re::parse(html, re_link_define_with_title, | capture: Captures | {
    // Prevent link titles from being defined improperly
    validate_title_definitions(&capture[3], &capture[5], "links");
    // Extract the link identifier, href and title
    let (href, title): (String, String) = (capture[2].trim().to_string(), capture[4].trim().to_string());
    // Determine if the link is a footnote and extract the identifier
    let (identifier, is_footnote): (String, bool) = get_footnote(capture[1].trim().to_string());
    // Add the link to the links vector
    links.insert(identifier, Link { href, title, is_footnote });
    // Remove the link definition
    String::new()
  });

  // Link definition (without title):
  let re_link_define: Regex = re::from(re::LINK_DEFINE);
  re::parse(html, re_link_define, | capture: Captures | {
    // Extract the link identifier, href and title
    let href: String = capture[2].trim().to_string();
    // Determine if the link is a footnote and extract the identifier
    let (identifier, is_footnote): (String, bool) = get_footnote(capture[1].trim().to_string());
    // Add the link to the links vector
    links.insert(identifier, Link { href, title: String::new(), is_footnote });
    // Remove the link definition
    String::new()
  });

  // Images (with title):
  let re_image_with_title: Regex = re::from(re::IMAGE_WITH_TITLE);
  re::parse(html, re_image_with_title, | capture: Captures | {
    // Prevent link titles from being defined improperly
    validate_title_definitions(&capture[3], &capture[5], "images");
    // Return the formatted HTML string (using a predefined link or the captured links)
    let (src, alt): (String, &str) = (capture[2].trim().to_string(), &capture[1].trim());
    let defined: String = format!("<img src=\"{}\" title=\"{}\" alt=\"{}\" />", &src, &capture[4].trim(), alt);
    html_from_link(&links, src, defined, | link: &Link | -> String {
      if !link.title.is_empty() {
        format!("<img src=\"{}\" title=\"{}\" alt=\"{}\" />", link.href, link.title, alt)
      } else {
        format!("<img src=\"{}\" alt=\"{}\" />", link.href, alt)
      }
    })
  });

  // Images (without title):
  let re_image: Regex = re::from(re::IMAGE);
  re::parse(html, re_image, | capture: Captures | {
    // Return the formatted HTML string (using a predefined link or the captured links)
    let (src, alt): (String, &str) = (capture[2].trim().to_string(), &capture[1].trim());
    let defined: String = format!("<img src=\"{src}\" alt=\"{alt}\" />", src = src, alt = alt);
    html_from_link(&links, src, defined, | link: &Link | -> String {
      if !link.title.is_empty() {
        format!("<img src=\"{src}\" alt=\"{alt}\" title=\"{title}\" />", src = link.href, alt = alt, title = link.title)
      } else {
        format!("<img src=\"{src}\" alt=\"{alt}\" />", src = link.href, alt = alt)
      }
    })
  });

  // Hyperlinks (with title):
  let re_link_with_title: Regex = re::from(re::LINK_WITH_TITLE);
  re::parse(html, re_link_with_title, | capture: Captures | {
    // Prevent link titles from being defined improperly
    validate_title_definitions(&capture[3], &capture[5], "images");
    // Return the formatted HTML string (using a predefined link or the captured links)
    let (href, text): (String, &str) = (capture[2].trim().to_string(), &capture[1].trim());
    let defined: String = format!("<a href=\"{}\" title=\"{}\">{}</a>", &href, &capture[4].trim(), text);
    html_from_link(&links, href, defined, | link: &Link | -> String {
      if !link.title.is_empty() {
        format!("<a href=\"{}\" title=\"{}\">{}</a>", link.href, link.title, text)
      } else {
        format!("<a href=\"{}\">{}</a>", link.href, text)
      }
    })
  });

  // Hyperlinks (without title):
  let re_link: Regex = re::from(re::LINK);
  re::parse(html, re_link, | capture: Captures | {
    // Return the formatted HTML string (using a predefined link or the captured links)
    let (href, text): (String, &str) = (capture[2].trim().to_string(), &capture[1].trim());
    let defined: String = format!("<a href=\"{href}\">{text}</a>", href = href, text = text);
    html_from_link(&links, href, defined, | link: &Link | -> String {
      if !link.title.is_empty() {
        format!("<a href=\"{href}\" title=\"{title}\">{text}</a>", href = link.href, text = text, title = link.title)
      } else {
        format!("<a href=\"{href}\">{text}</a>", href = link.href, text = text)
      }
    })
  });

  // Footnotes:
  let re_footnote: Regex = re::from(re::FOOTNOTE);
  re::parse(html, re_footnote, | capture: Captures | {
    let index: String = capture[1].trim().to_string();
    if links.contains_key(&index) {
      let link: &Link = links.get(&index).unwrap();
      if link.is_footnote == true {
        return match Url::parse(&link.href) {
          Ok(_) => format!("<sup><a href=\"{}\">[{}]</a></sup>", link.href, index),
          Err(_) => format!("<sup title=\"{}\" style=\"cursor:help\">[{}]</sup>", link.href, index)
        };
      }
    }
    return format!("<sup style=\"cursor:not-allowed\">[{}]</sup>", index);
  });
}
