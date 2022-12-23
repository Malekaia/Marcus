# Marcus
## Description:
[Marcus](https://crates.io/crates/marcus) is an experimental Markdown parser written in Rust. It uses [regular expressions](https://docs.rs/regex/latest/regex/#example-iterating-over-capture-groups) and built-in methods (for the [str](https://doc.rust-lang.org/std/primitive.str.html#implementations) &amp; [String](https://doc.rust-lang.org/std/string/struct.String.html#implementations) types) to convert Markdown into HTML. Please note that certain test files (`*.md`) aren't rendered properly by GitHub Preview due to a lack of support for the full Markdown specification.

## Demonstration:
Include the following dependencies in the `[dependencies]` section of the `Cargo.toml` file:

```rust
glob = "0.3.0"
marcus = "0.1.1"
```

The Marcus crate can also be included from the [GitHub repository](https://github.com/Malekaia/Marcus):

```rust
glob = "0.3.0"
marcus = { git = "https://github.com/Malekaia/Marcus" }
```

For this demonstration, the [`fs`](https://doc.rust-lang.org/std/fs/) and [`glob`](https://github.com/rust-lang/glob) crates are used to convert sample files into HTML:

```rust
use glob::glob;
use marcus;
use std::fs;

fn main() {

  // Iterate the globbed paths
  for path_buf in glob("./test/**/*.md").expect("GlobError: Failed to read glob pattern") {

    // Get the file path from the glob entry
    let file_path: String = path_buf.expect("GlobError: failed to glob entry").display().to_string();

    // Create the HTML output path
    let output_path: &String = &file_path.replace(".md", ".html");

    // Read the MarkDown from the globbed file
    let md: String = fs::read_to_string(&file_path).expect("ReadError: failed to read file");

    // Convert the HTML to MD using Marcus
    let html: String = marcus::to_string(md);

    // Write the HTML to the HTML output path
    fs::write(output_path, html).expect("WriteError: failed to write to file");
  }
}
```

**Note**: The test files (`.md`) used in this demonstration and their corresponding outputs (`.html`) are available to download in the [docs folder](https://github.com/Malekaia/Marcus/tree/main/docs).

## Licence:
The source code included in this repository is distributed, for free, under the [MIT Licence](https://choosealicense.com/licenses/mit/), for the full license, see [LICENSE.md](https://github.com/Malekaia/Marcus/blob/master/LICENSE.md).
