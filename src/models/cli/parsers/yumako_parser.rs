use std::collections::HashMap;

use lyn::Action;
use lyn::Scanner;
use serde_json::Value;

fn variable(scanner: &mut Scanner) -> Result<String, lyn::Error> {
    scanner
        .scan(|symbol| {
            // Check for escaped =
            if symbol.ends_with("\\=") {
                return Some(Action::Require);
            }

            if symbol.ends_with("=") {
                let mut out = symbol.to_string();
                out.pop();
                let out = out.replace("\\=", "=");
                let out = out.trim().to_string();
                return Some(Action::Return(out));
            }

            Some(Action::Require)
        })
        .transpose()
        .expect("The scan doesn't return None")
}

fn data(scanner: &mut Scanner) -> Result<String, lyn::Error> {
    scanner
        .scan(|symbol| {
            // Check for escaped =
            if symbol.ends_with("\\;") {
                return Some(Action::Request(symbol.to_string()));
            }

            if symbol.ends_with(";") {
                let mut out = symbol.to_string();
                out.pop();
                let out = out.replace("\\;", ";");
                let out = out.trim().to_string();
                return Some(Action::Return(out));
            }

            Some(Action::Request(symbol.to_string()))
        })
        .transpose()
        .expect("The scan doesn't return None")
}

fn key_value(scanner: &mut Scanner) -> Result<(String, String), lyn::Error> {
    Ok((variable(scanner)?, data(scanner)?))
}

fn key_values(scanner: &mut Scanner) -> Result<Vec<(String, String)>, lyn::Error> {
    let mut key_values = Vec::new();

    while !scanner.is_done() {
        key_values.push(key_value(scanner)?)
    }

    Ok(key_values)
}

pub fn parse_yumako_variables(string: &str) -> Result<HashMap<String, Value>, crate::Error> {
    let mut scanner = Scanner::new(string);

    let mut out = HashMap::new();

    for (key, value) in
        key_values(&mut scanner).map_err(|_| crate::Error::YumakoArgumentParsingError())?
    {
        out.insert(
            key.clone(),
            serde_json::from_str(&value).map_err(|err| {
                crate::Error::YumakoArgumentDataDeserializingError(key, value, err)
            })?,
        );
    }

    Ok(out)
}

#[test]
fn wwooo_test() {
    let mut scanner = Scanner::new("test=true; onl = \"Hello\"; eq\\=escape=oop");

    assert_eq!(
        key_values(&mut scanner),
        Ok(vec![
            ("test".to_string(), "true".to_string()),
            ("onl".to_string(), "\"Hello\"".to_string()),
            ("eq=escape".to_string(), "oop".to_string())
        ])
    );
}
