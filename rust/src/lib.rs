use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

#[pyfunction]
fn lexer(code: &str) -> PyResult<Vec<(String, String)>> {
    let mut tokens = Vec::new();
    let mut chars = code.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => continue,
            '=' => tokens.push(("ASSIGN".into(), "=".into())),
            '+' => tokens.push(("OP".into(), "+".into())),
            '"' => {
                let mut s = String::from("\"");
                loop {
                    match chars.next() {
                        Some((_, '"')) => { s.push('"'); break; }
                        Some((_, ch)) => s.push(ch),
                        None => return Err(PyRuntimeError::new_err("Unterminated string")),
                    }
                }
                tokens.push(("STRING".into(), s));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut word = String::from(c);
                while let Some(&(_, nc)) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        word.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = match word.as_str() {
                    "print" => "PRINT",
                    "let"   => "LET",
                    _       => "ID",
                };
                tokens.push((kind.into(), word));
            }
            other => {
                return Err(PyRuntimeError::new_err(format!("Unexpected character: {other}")));
            }
        }
    }

    Ok(tokens)
}
#[pyfunction]
fn transpile(tokens: Vec<(String, String)>) -> PyResult<String> {
    let mut out = String::from("<?php\n\n");
    let mut pos = 0;

    while pos < tokens.len() {
        let (kind, _) = &tokens[pos];
        match kind.as_str() {
            "LET" => {
                if pos + 3 >= tokens.len() {
                    return Err(PyRuntimeError::new_err("Incomplete let statement"));
                }
                let var_name = &tokens[pos + 1];
                let assign   = &tokens[pos + 2];
                let val      = &tokens[pos + 3];

                if var_name.0 != "ID" {
                    return Err(PyRuntimeError::new_err(format!("Expected variable name after 'let', got '{}'", var_name.1)));
                }
                if assign.0 != "ASSIGN" {
                    return Err(PyRuntimeError::new_err(format!("Expected '=' after '{}', got '{}'", var_name.1, assign.1)));
                }
                if val.0 != "STRING" && val.0 != "ID" {
                    return Err(PyRuntimeError::new_err(format!("Expected value after '=', got '{}'", val.1)));
                }

                let rhs = if val.0 == "ID" { format!("${}", val.1) } else { val.1.clone() };
                out.push_str(&format!("${} = {};\n", var_name.1, rhs));
                pos += 4;
            }
            "PRINT" => {
                if pos + 1 >= tokens.len() {
                    return Err(PyRuntimeError::new_err("Expected value after 'print'"));
                }
                let val = &tokens[pos + 1];
                if val.0 != "STRING" && val.0 != "ID" {
                    return Err(PyRuntimeError::new_err(format!("Expected variable or string after 'print', got '{}'", val.1)));
                }

                let rhs = if val.0 == "ID" { format!("${}", val.1) } else { val.1.clone() };
                out.push_str(&format!("echo {};\n", rhs));
                pos += 2;
            }
            other => {
                return Err(PyRuntimeError::new_err(format!("Unexpected token '{}'", other)));
            }
        }
    }

    Ok(out)
}


#[pymodule]
fn phplus_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lexer, m)?)?;
    m.add_function(wrap_pyfunction!(transpile, m)?)?;
    Ok(())
}
