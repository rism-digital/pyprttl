# pyprttl

Python bindings for the Rust `prttl` Turtle formatter.

```python
import pyprttl

formatted = pyprttl.format_turtle("@prefix ex: <http://example.com/> . ex:s ex:p ex:o .")
```

The package is built with maturin and depends on the sibling Rust checkout at
`../prttl`.
