# Marcus
## Description:
[Marcus](https://crates.io/crates/marcus) is an experimental Markdown parser written in Rust. It uses [regular expressions](https://docs.rs/regex/latest/regex/#example-iterating-over-capture-groups) and built-in methods (for the [str](https://doc.rust-lang.org/std/primitive.str.html#implementations) &amp; [String](https://doc.rust-lang.org/std/string/struct.String.html#implementations) types) to convert Markdown into HTML.

**Note**: Certain test files (`*.md`) aren't rendered properly by Github Preview due to a lack of support for the full Markdown specification.

**Warning**: This project is experimental and isn't production tested. Make sure to backup all files when working with file I/O.

## Demonstration:
Include the following dependencies in the `[dependencies]` section of the `Cargo.toml` file:

```rust
glob = "0.3.0"
marcus = "0.1.0"
```

The Marcus package can also be included from the [Github repository](https://github.com/Malekaia/Marcus):

```rust
glob = "0.3.0"
marcus = { git = "https://github.com/Malekaia/Marcus" }
```

The following helper functions are used to simplify file I/O and to improve error handling.

```rust
use glob::glob;
use std::fs;

pub fn read_file(file_path: &str) -> String {
  match fs::read_to_string(file_path) {
    Ok(file_content) => file_content,
    Err(_) => panic!("ReadError: failed to read \"{file_path}\"")
  }
}

pub fn write_file(file_path: &str, contents: String) {
  fs::write(file_path, contents).expect("WriteError: failed to write to file");
}

pub fn walk(path: &str) -> impl Iterator<Item = String> {
  glob(path)
    .expect("GlobError: Failed to read glob pattern")
    .map(| entry |
      entry.expect("GlobError: failed to glob entry").display().to_string()
    )
}
```

Define the `fileio` module (contains the helper functions above), and import the `marcus` package:

```rust
mod fileio;
use marcus;

fn main() {
  // Nothing here yet...
}
```

In the body of the `main` function, use `fileio::walk` to get a list of file paths and read the contents of each file using `fileio::read_file`. Then convert each file's contents to HTML using `marcus::to_string` and use `fileio::write_file` to store the HTML result to a `.html` file of the same name.

```rust
// Glob the MD test files
for file_path in fileio::walk("./test/**/*.md") {
  // Read the MD from file
  let md: String = fileio::read_file(&file_path);
  // Convert the MD to HTML
  let html: String = marcus::to_string(md);
  // Write to file
  fileio::write_file(&file_path.replace(".md", ".html"), html)
}
```

**Note**: The test files (`.md`) used in this demonstration and their corresponding outputs (`.html`) are available to download on the [Marcus-Docs](https://github.com/Malekaia/Marcus-Docs) repository.

## Licence:
The source code included in this repository is distributed, for free, under the [MIT Licence](https://choosealicense.com/licenses/mit/), for the full license, see [LICENSE.md](https://github.com/Malekaia/Marcus/blob/master/LICENSE.md).
