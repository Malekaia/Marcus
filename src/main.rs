mod lib;
mod parser;
mod regex;

fn main() {
  let mut html: String = r#"> Level 1
>> Level 1.1
>> Level 1.2

Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit

>>> Level 3
> Level 3-X
> Level 3-Y -- Cite
> Level 3-Z -- Cite -- Cite 2 -- Cite 3

Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit

> Level 4
>> Level 5
>>> Level 6

Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit

> Level 8
>> Level 9
>>> Level 10

Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit

> Note: `--capt-add=SYS-ADMIN` is required for PDF rendering.
"#.to_string();

  const ALLOW_COMMENTS: bool = false;


  parser::escape::default(&mut html);
  parser::comments::default(&mut html, ALLOW_COMMENTS);
  parser::blockquotes::default(&mut html);

  // println!("{html}");


}
