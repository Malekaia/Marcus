This example demonstrates four types of footnotes:

<ol>
  <li>A line with a footnote[^1].</li>
  <li>A line with a duplicate footnote[^1].</li>
  <li>A line with a footnote[^2] that is title only (I.E: no valid link/href).</li>
  <li>A line with an undefined footnote[^3].</li>
</ol>

[^1]: https://example.com
[^2]: This is the footnote.

Here is a list explaining the behaviour of each type of footnote:

<ul>
  <li>The footnote with a valid href opens the link in a new tab</li>
  <li>The footnote with a title only text uses the title attribute with a [`question mark`](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#try_it) cursor to indicate that the user should hover over it</li>
  <li>The footnote with an undefined reference does nothing (I.E: it's plain text), and has a `not allowed` cursor</li>
  <li>Duplicate footnotes are parsed exactly the same as the original</li>
</ul>

<p>**Note**: How the plain HTML formatting of this example is ignored</p>
