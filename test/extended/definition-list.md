<p style="color: blue">This is a definition list with varying whitespace:</p>

First Term
: This is the definition of the first term.

Second Term
: This is one definition of the second term.




: This is another definition of the second term.


<p style="color: blue">Here is a definition list with even more whitespace</p>


First Term
: This is the definition of the first term.













Second Term
: This is one definition of the second term.




: This is another definition of the second term.


<p style="color: blue">Those definition lists should then output the following HTML:</p>

```html
\<dl\>
  \<dt\>First Term\<\/dt\>
  \<dd\>This is the definition of the first term.\<\/dd\>
  \<dt\>Second Term\<\/dt\>
  \<dd\>This is one definition of the second term.\<\/dd\>
  \<dd\>This is another definition of the second term.\<\/dd\>
\<\/dl\>
```

<p style="color: blue">Notice how the HTML is escaped and has a language specifier in the source code?</p>
