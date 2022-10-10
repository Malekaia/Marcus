use crate::core::re;
use regex::{Captures, Regex};

// Table type shorthands to improve visual grepping of table components
type Cell = String;
type Row = Vec<Cell>;
type Table = Vec<Row>;
type Thead = Table;
type Tbody = Table;
type Tfoot = Table;

// Convert a table section to a HTML string
fn section_to_html(section: Table, name: &str) -> String {
  let mut html: String = format!("  <t{name}>\n");
  for row in section {
    html.push_str("    <tr>\n");
    for cell in row {
      let format: &str = if name == "head" || name == "foot" { "th" } else { "td" };
      html.push_str(&format!("      <{format}>{}</{format}>\n", cell));
    }
    html.push_str("    </tr>\n");
  }
  html.push_str(&format!("  </t{name}>\n"));
  html
}

// Parse: Tables
pub fn default(html: &mut String) {
  let re_table: Regex = re::from(re::TABLE);
  re::parse(html, re_table, | capture: Captures | {
    // Split the table capture into lines
    let table: Table = capture[0].trim().lines()
      // Collect the cells
      .map(| mut line: &str | {
        // Remove the first and last pipe
        line = &line.trim();
        if line.starts_with("|") {
          line = &line.trim()[1..];
        }
        if line.ends_with("|") {
          line = &line[0..(line.len() - 1)];
        }
        // Split the cells
        line.trim().split("|")
          // Collect the cells and create the table rows
          .map(| cell: &str | cell.replace(":", "").replace("-", "").trim().to_string())
          // Collect the cells into a row
          .collect::<Row>()
      })
      // Filter out empty and separator cells
      .map(| mut row: Row | if row.join("").trim().is_empty() { row.clear(); row } else { row })
      // Collect the table as a vector of rows (which is a vector of cells)
      .collect::<Table>();

    // The length of the largest row in the table
    let mut max: usize = 0;
    // The current index and the table's length
    let (mut i, table_length): (usize, usize) = (0, table.len());
    // Sort the table into header, body and footer
    let (mut table_header, mut table_body, mut table_footer): (Thead, Tbody, Tfoot) = (vec![], vec![], vec![]);
    // The current section the rows are being moved to
    let mut current_section: &str = "header";

    // Iterate the table
    for row in table {
      // Update the largest row length (used to pad the table later)
      let row_length: usize = row.len();
      if max < row_length {
        max = row_length;
      }
      // Ignore the first and last items if they're empty / separator cells
      if !((i == 0 && row_length < 1) || (i == (table_length - 1) && row_length < 1)) {
        // Move to the next section on empty rows
        if row_length < 1 {
          current_section = match current_section { "header" => "body", "body" => "footer", _ => "footer" };
        }
        // Add the row to the current section and continue
        else {
          match current_section {
            "header" => table_header.push(row),
            "body" => table_body.push(row),
            _ => table_footer.push(row)
          }
        }
      }
      // Increment the index
      i += 1;
    }

    // Determine output structure for table
    let mut body: Tbody;
    let (mut header, mut footer): (Thead, Tfoot) = (vec![], vec![]);
    // 0, 0, 0 (no header, no body, no footer) => Ignore
    if table_header.is_empty() && table_body.is_empty() && table_footer.is_empty() {
      return String::new()
    }
    // 1, 0, 0 (header, no body, no footer) => Body
    else if !table_header.is_empty() && table_body.is_empty() && table_footer.is_empty() {
      body = table_header;
    }
    // 1, 1, 0 (header, body, no footer) => Header and body
    else if !table_header.is_empty() && !table_body.is_empty() && table_footer.is_empty() {
      (header, body) = (table_header, table_body);
    }
    // 1, 1, 1 (header, body, footer) => Header, body and footer
    else if !table_header.is_empty() && !table_body.is_empty() && !table_footer.is_empty() {
      (header, body, footer) = (table_header, table_body, table_footer);
    }
    // 0, 0, 1 (no header, no body, footer) => Body
    else if table_header.is_empty() && table_body.is_empty() && !table_footer.is_empty() {
      body = table_footer;
    }
    // 0, 1, 1 (no header, body, footer) => Body and footer
    else if table_header.is_empty() && !table_body.is_empty() && !table_footer.is_empty() {
      (body, footer) = (table_body, table_footer);
    }
    // 1, 0, 1 (header, no body, footer) => Header and body
    else if !table_header.is_empty() && table_body.is_empty() && !table_footer.is_empty() {
      (header, body) = (table_header, table_footer);
    }
    // 0, 1, 0 (no header, body, no footer) => Body
    else if !table_header.is_empty() && table_body.is_empty() && !table_footer.is_empty() {
      body = table_body;
    }
    // Anything else => Ignore
    else {
      (header, body, footer) = (table_header, table_body, table_footer);
    }

    // Correct header and footer sizes
    while header.len() > 1 {
      body.insert(0, header.pop().unwrap());
    }
    while footer.len() > 1 {
      body.insert(body.len(), footer.remove(0));
    }

    // Return the formatted HTML table
    format!(
      "\n<table>\n{}{}{}</table>\n",
      section_to_html(header, "head"),
      section_to_html(body, "body"),
      section_to_html(footer, "foot")
    )
  });
}
