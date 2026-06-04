# pyprttl

Python bindings for the Rust [`prttl`](https://codeberg.org/elevont/prttl) Turtle formatter.

## Installation

```sh
pip install pyprttl
```

Python 3.11 or newer is required. If a wheel is not available for your
platform, pip will build from the source distribution. Source builds require a
Rust toolchain, Cargo, and git access so Cargo can fetch the pinned `prttl`
dependency.

## Usage

```python
import pyprttl

formatted = pyprttl.format_turtle("@prefix ex: <http://example.com/> . ex:s ex:p ex:o .")
```

The package is built with maturin.
