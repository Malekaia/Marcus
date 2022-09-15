mod lib;
mod parser;
mod regex;

fn main() {
  let mut html: String = r#"\> Level 1
>> Level 2
> Level 1.1

Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit

>>> Level \3
> Level 1.1
> Level 1.1 Cite

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

  parser::escape::default(&mut html);

  println!("{html}");


}
