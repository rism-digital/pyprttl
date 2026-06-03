import pytest

import pyprttl


def test_format_turtle_simple():
    output = pyprttl.format_turtle(
        "@prefix ex: <http://example.com/> .\n\nex:s ex:p ex:o .\n",
        canonicalize=False,
    )

    assert "@prefix ex: <http://example.com/> ." in output
    assert "ex:s" in output
    assert output.endswith("\n")


def test_invalid_turtle_raises_prttl_error():
    with pytest.raises(pyprttl.PrttlError):
        pyprttl.format_turtle("@prefix ex: <http://example.com/> .\nex:s ex:p .")


def test_comments_require_force():
    source = "@prefix ex: <http://example.com/> .\n# comment\nex:s ex:p ex:o .\n"

    with pytest.raises(pyprttl.PrttlError):
        pyprttl.format_turtle(source)

    assert "# comment" not in pyprttl.format_turtle(source, force=True)


def test_indentation_integer_controls_indent_width():
    output = pyprttl.format_turtle(
        "@prefix ex: <http://example.com/> .\nex:s ex:p ex:o ; ex:p2 ex:o2 .\n",
        canonicalize=False,
        indentation=4,
    )

    assert "\n    ex:p ex:o ;" in output


def test_predicate_order_preset_is_accepted():
    output = pyprttl.format_turtle(
        "@prefix ex: <http://example.com/> .\nex:s ex:p ex:o .\n",
        canonicalize=False,
        predicate_order_preset="rdf",
    )

    assert "ex:s" in output
