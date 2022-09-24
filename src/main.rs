mod lib;
mod parser;
mod regex;

fn main() {
  let mut html: String = r#"Neque porro **quisquam** est qui dolorem ***ipsum*** quia dolor sit amet, *consectetur*, adipisci velit

Bold: **bold text**, Italic: *italic text*, Bold Italic: ***bold italic text***

Marker also supports emphasis: *this text will be italic*, _this will also be italic_, **this text will be bold**, __This will also be bold__, _you **can** also **combine** them and have *multiple* on one line_.

For example, ___this will be bold italic___ and ***this will also be bold italic***."#.to_string();

  const ALLOW_COMMENTS: bool = false;

  parser::escape::default(&mut html);
  parser::comments::default(&mut html, ALLOW_COMMENTS);
  parser::blockquotes::default(&mut html);
  parser::inline_code::default(&mut html);
  parser::emphasis::default(&mut html);

  println!("{html}");


}
