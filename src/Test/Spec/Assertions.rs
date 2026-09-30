// Failure messages: a best-effort JSON rendering of native values.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn render(value: &crate::UnknownType, depth: usize) -> String {
    if depth > 6 {
        return "\"...\"".to_owned();
    }
    // Unboxed Int arrays render like regular arrays. Resolve once and keep a
    // reference so every branch below sees the same type.
    let converted;
    let resolved: &crate::Value = match value.resolve() {
        crate::Value::IntArray(_) | crate::Value::NativeArray(_) => {
            converted = crate::Value::Array(value.unwrap_array());
            &converted
        }
        other => other,
    };
    match resolved {
        crate::Value::String(text) => escape(text.as_str()),
        crate::Value::Int(number) => number.to_string(),
        crate::Value::Number(number) => number.to_string(),
        crate::Value::Bool(flag) => flag.to_string(),
        crate::Value::Char(character) => escape(&character.to_string()),
        crate::Value::Array(values) => {
            let items: Vec<String> = values
                .iter()
                .map(|value| render(value, depth + 1))
                .collect();
            format!("[{}]", items.join(","))
        }
        _ => match resolved.__purust_record_fields() {
            Some(fields) => {
                let items: Vec<String> = fields
                    .entries()
                    .into_iter()
                    .map(|(key, value)| format!("{}:{}", escape(&key), render(&value, depth + 1)))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
            None => "\"<native>\"".to_owned(),
        },
    }
}

pub fn Test_Spec_Assertions_unsafeStringify(value: crate::UnknownType) -> String {
    render(&value, 0)
}
