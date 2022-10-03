(function() {
  // List the test file names
  let files = {
    "basic": [
      "blockquote", "code", "comments", "emphasis", "heading",
      "horizontal-rule", "image", "link", "list", "paragraphsd"
    ],
    "custom": [
      "auto-email", "inline-ignore"
    ],
    "extended": [
      "definition-list", "emoji", "fenced-code-block", "footnote",
      "heading-id", "highlight", "strikethrough", "subscript",
      "superscript", "table", "task-list"
    ]
  };

  // Ajax file read utility
  function read_file(file_path, callback) {
    var request = new XMLHttpRequest();
    request.open("GET", file_path, true);
    request.onreadystatechange = function() {
      if (request.readyState === 4 && (request.status === 200 || request.status == 0)) {
        let response_text = request.responseText;
        callback(response_text);
      }
    };
    request.send(null);
  }

  // Create the option element
  function create_option(parent, file_name, category) {
    // Create the option element
    let option = document.createElement("option");
    option.innerHTML = file_name;
    option.setAttribute("value", file_name);
    option.setAttribute("data-category", category);
    // Add the option element to the parent select
    parent.appendChild(option);
  }

  // Cache the preview elements
  let [preview_md, preview_html] = [document.querySelector("#preview-md"), document.querySelector("#preview-html")];

  // Switch the view components
  function switch_view() {
    // Get the category, file name and file paths
    let [category, file_name] = [this.getAttribute("data-category"), this.value];
    let [path_md, path_html] = [`./${category}/${file_name}.md`, `./${category}/${file_name}.html`];
    // Update the preview blocks
    read_file(path_md, response_text => preview_md.textContent = response_text);
    read_file(path_html, response_text => preview_html.textContent = response_text);
  }

  // Iter the test file data
  for (const [category, file_list] of Object.entries(files)) {
    // Get the parent (select) element for the options
    let parent = document.querySelector(`select#${category}`);
    // Add the category to the parent element
    parent.setAttribute("data-category", category);
    // Create and append the new option to the parent
    file_list.forEach(file_name => create_option(parent, file_name, category));
    // Add the switching event listener to the parent
    parent.addEventListener("change", switch_view, false);
  }
})();
