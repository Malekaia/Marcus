use glob::glob;
use marcus;
use std::fs;

fn main() {
  // Iterate the globbed paths
  for path_buf in glob("./examples/**/*.md").expect("GlobError: Failed to read glob pattern") {
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
