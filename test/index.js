(function() {
  // Cache the preview elements and create an object for the test file categories and names
  const [preview_md, preview_html, files] = [
    document.querySelector("#preview-md"),
    document.querySelector("#preview-html"),
    {
      "basic": [
        "blockquote", "code", "comments", "emphasis", "heading",
        "horizontal-rule", "image", "link", "list", "paragraphs"
      ],
      "custom": [
        "auto-email", "inline-ignore"
      ],
      "extended": [
        "definition-list", "emoji", "fenced-code-block", "footnote",
        "heading-id", "highlight", "strikethrough", "subscript",
        "superscript", "table", "task-list"
      ]
    }
  ];

  // Ajax file read utility
  function read_file(file_path, callback) {
    var request = new XMLHttpRequest();
    request.open("GET", file_path, true);
    request.onreadystatechange = function() {
      if (request.readyState === 4 && (request.status === 200 || request.status == 0)) {
        const response_text = request.responseText;
        callback(response_text);
      }
    };
    request.send(null);
  }

  // Switch the view components
  function switch_view() {
    // Get the category, file name and file paths
    const [category, file_name] = [this.getAttribute("data-category"), this.value];
    const [path_md, path_html] = [`./${category}/${file_name}.md`, `./${category}/${file_name}.html`];
    // Update the preview blocks
    read_file(path_md, response_text => preview_md.textContent = response_text);
    read_file(path_html, response_text => preview_html.textContent = response_text);
  }

  // Loop through the test file data
  for (const [category, file_list] of Object.entries(files)) {
    // Get the parent (select) element for the options
    const parent = document.querySelector(`select#${category}`);
    // Add the category to the parent element
    parent.setAttribute("data-category", category);
    // Loop through the file list
    for (const file_name of file_list) {
      // Create the option element
      const option = document.createElement("option");
      option.innerHTML = file_name;
      option.setAttribute("value", file_name);
      option.setAttribute("data-category", category);
      // Add a click event to switch the view to the option element
      option.addEventListener("click", () => switch_view.apply(parent, []));
      // Add the option element to the parent select
      parent.appendChild(option);
    }
    // Add the switching event listener to the parent
    parent.addEventListener("change", switch_view, false);
  }
})();
