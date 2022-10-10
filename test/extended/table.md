<p><b>0, 0, 0</b>: no header, no body, no footer => Ignore</p>

| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |


<p><b>1, 0, 0</b>: header, no body, no footer => Body</p>

| ------------- |:-------------:| --------------- |
| Left columns  | Right columns | Optional column |
| left col 1    | right col 1   |
| left col 2    |               | Optional value  |
|               | right col 3   |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |

<p><b>1, 1, 0</b>: header, body, no footer => Header and body</p>

| ------------- |:-------------:| --------------- |
| Left columns  | Right columns | Optional column |
| ------------- |:-------------:| --------------- |
| left col 1    | right col 1   |
| left col 2    |               | Optional value  |
|               | right col 3   |
| Left columns  | Right columns |                 |
| ------------- |:-------------:| --------------- |

<p><b>1, 1, 1</b>: header, body, footer => Header, body and footer</p>

| ------------- |:-------------:| --------------- |
| Left columns  | Right columns | Optional column |
| ------------- |:-------------:| --------------- |
| left col 1    | right col 1   |
| left col 2    |               | Optional value  |
|               | right col 3   |
| ------------- |:-------------:| --------------- |
| Left columns  | Right columns |                 |
| ------------- |:-------------:| --------------- |

<p><b>0, 0, 1</b>: no header, no body, footer => Body</p>

| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
|               | right col 3   |
| Left columns  | Right columns |                 |
| ------------- |:-------------:| --------------- |

<p><b>0, 1, 1</b>: no header, body, footer => Body and footer</p>

| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| Left columns  | Right columns | Optional column |
| left col 1    | right col 1   |
| left col 2    |               | Optional value  |
|               | right col 3   |
| ------------- |:-------------:| --------------- |
| Left columns  | Right columns |                 |
| ------------- |:-------------:| --------------- |

<p><b>1, 0, 1</b>: header, no body, footer => Header and body</p>

| ------------- |:-------------:| --------------- |
| left col 2    |               | Optional value  |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
|               | right col 3   |
| ------------- |:-------------:| --------------- |

<p><b>0, 1, 0</b>: no header, body, no footer => Body</p>

| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |
| left col 2    |               | Optional value  |
|               | right col 3   |
| left col 2    |               | Optional value  |
|               | right col 3   |
| ------------- |:-------------:| --------------- |
| ------------- |:-------------:| --------------- |

<style type="text/css">
  table, th, td {
    border: 1px solid black;
    border-collapse: collapse;
  }
</style>
