# pyprttl

Python bindings for the Rust [`prttl`](https://codeberg.org/elevont/prttl) Turtle formatter.

## Installation

Python 3.11 or newer is required.

Install the published package from PyPI:

```sh
pip install pyprttl
```

If a wheel is not available for your platform, `pip` will build from source.
Source builds require:

- Python 3.11+
- a Rust toolchain with `cargo`
- `git`, because Cargo fetches the pinned `prttl` dependency from its upstream git repository

To install from a local checkout:

```sh
pip install .
```

To install in editable mode for development:

```sh
uv sync --group dev
uv pip install -e '.[test]'
```

This creates `.venv/` and installs the build and test dependencies used by this
project.

## Building

Build an editable local extension module into the active virtual environment:

```sh
uv sync --group dev
.venv/bin/python -m maturin develop
```

Build a wheel:

```sh
uv sync --group dev
.venv/bin/python -m maturin build --release --locked --compatibility pypi
```

The wheel will be written to `target/wheels/`.

Build a source distribution:

```sh
uv sync --group dev
.venv/bin/python -m maturin sdist --out dist
```

## Testing

Run the test suite with:

```sh
.venv/bin/python -m pytest -q
```

## Usage

```python
import pyprttl

formatted = pyprttl.format_turtle("@prefix ex: <http://example.com/> . ex:s ex:p ex:o .")
```

The package is built with maturin.
