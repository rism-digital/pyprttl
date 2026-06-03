use std::rc::Rc;

use prttl::{
    formatter,
    options::{FormatOptions, SpecialPredicateOrder, SpecialSubjectTypeOrder},
    parser,
};
use pyo3::{
    create_exception,
    exceptions::{PyException, PyTypeError, PyValueError},
    prelude::*,
    types::PyAny,
};

create_exception!(pyprttl, PrttlError, PyException);

fn parse_subject_type_order_preset(
    value: Option<&str>,
) -> PyResult<Option<SpecialSubjectTypeOrder>> {
    value
        .map(|value| match value {
            "owl" => Ok(SpecialSubjectTypeOrder::Owl),
            "skos" => Ok(SpecialSubjectTypeOrder::Skos),
            "shacl" => Ok(SpecialSubjectTypeOrder::Shacl),
            "shex" => Ok(SpecialSubjectTypeOrder::Shex),
            "rdf" => Ok(SpecialSubjectTypeOrder::Rdf),
            _ => Err(PyValueError::new_err(format!(
                "unknown subject_type_order_preset: {value}"
            ))),
        })
        .transpose()
}

fn parse_predicate_order_preset(value: Option<&str>) -> PyResult<Option<SpecialPredicateOrder>> {
    value
        .map(|value| match value {
            "owl" => Ok(SpecialPredicateOrder::Owl),
            "skos" => Ok(SpecialPredicateOrder::Skos),
            "shacl" => Ok(SpecialPredicateOrder::Shacl),
            "shex" => Ok(SpecialPredicateOrder::Shex),
            "rdf" => Ok(SpecialPredicateOrder::Rdf),
            _ => Err(PyValueError::new_err(format!(
                "unknown predicate_order_preset: {value}"
            ))),
        })
        .transpose()
}

fn parse_indentation(value: Option<&Bound<'_, PyAny>>) -> PyResult<String> {
    match value {
        None => Ok(FormatOptions::default().indentation),
        Some(value) => {
            if let Ok(count) = value.extract::<usize>() {
                if count == 0 {
                    return Err(PyValueError::new_err("indentation must be at least 1"));
                }
                Ok(" ".repeat(count))
            } else if let Ok(text) = value.extract::<String>() {
                if text.is_empty() {
                    return Err(PyValueError::new_err("indentation must not be empty"));
                }
                Ok(text)
            } else {
                Err(PyTypeError::new_err("indentation must be an int or str"))
            }
        }
    }
}

/// Formats a Turtle document and returns the formatted Turtle text.
#[pyfunction(signature = (
    text,
    *,
    indentation = None,
    single_leafed_new_lines = false,
    force = false,
    generate_sorting_ids = false,
    prioritize_input_order = false,
    prtr_sorting = true,
    sparql_syntax = false,
    max_nesting = true,
    canonicalize = true,
    warn_unsupported_numbers = true,
    subject_type_order = None,
    subject_type_order_preset = None,
    predicate_order = None,
    predicate_order_preset = None
))]
#[allow(clippy::too_many_arguments)]
fn format_turtle(
    text: &str,
    indentation: Option<&Bound<'_, PyAny>>,
    single_leafed_new_lines: bool,
    force: bool,
    generate_sorting_ids: bool,
    prioritize_input_order: bool,
    prtr_sorting: bool,
    sparql_syntax: bool,
    max_nesting: bool,
    canonicalize: bool,
    warn_unsupported_numbers: bool,
    subject_type_order: Option<Vec<String>>,
    subject_type_order_preset: Option<&str>,
    predicate_order: Option<Vec<String>>,
    predicate_order_preset: Option<&str>,
) -> PyResult<String> {
    let options = FormatOptions {
        check: false,
        indentation: parse_indentation(indentation)?,
        single_leafed_new_lines,
        force,
        generate_sorting_ids,
        prioritize_input_order,
        prtr_sorting,
        sparql_syntax,
        max_nesting,
        canonicalize,
        warn_unsupported_numbers,
        subject_type_order_preset: parse_subject_type_order_preset(subject_type_order_preset)?,
        subject_type_order,
        predicate_order_preset: parse_predicate_order_preset(predicate_order_preset)?,
        predicate_order,
    };
    let options = Rc::new(options);
    let input = parser::parse(text.as_bytes(), &options)
        .map_err(|err| PrttlError::new_err(err.to_string()))?;
    formatter::format(&input, options).map_err(|err| PrttlError::new_err(err.to_string()))
}

#[pymodule]
fn pyprttl(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("PrttlError", module.py().get_type::<PrttlError>())?;
    module.add_function(wrap_pyfunction!(format_turtle, module)?)?;
    Ok(())
}
