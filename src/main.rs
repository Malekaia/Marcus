mod lib;
mod parser;
mod regex;

fn main() {
  let mut html: String = r#"This is some inline `code` which will be parsed into a HTML `\<code \/\>` element."#.to_string();

  const ALLOW_COMMENTS: bool = false;

  parser::escape::default(&mut html);
  parser::comments::default(&mut html, ALLOW_COMMENTS);
  parser::blockquotes::default(&mut html);
  parser::inline_code::default(&mut html);

  println!("{html}");


}
