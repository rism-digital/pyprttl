from typing import Literal

__version__: str


class PrttlError(Exception): ...


def format_turtle(
    text: str,
    *,
    indentation: int | str | None = None,
    single_leafed_new_lines: bool = False,
    force: bool = False,
    generate_sorting_ids: bool = False,
    prioritize_input_order: bool = False,
    prtr_sorting: bool = True,
    sparql_syntax: bool = False,
    max_nesting: bool = True,
    canonicalize: bool = True,
    warn_unsupported_numbers: bool = True,
    subject_type_order: list[str] | None = None,
    subject_type_order_preset: (
        Literal["owl", "skos", "shacl", "shex", "rdf"] | None
    ) = None,
    predicate_order: list[str] | None = None,
    predicate_order_preset: (
        Literal["owl", "skos", "shacl", "shex", "rdf"] | None
    ) = None,
) -> str: ...
