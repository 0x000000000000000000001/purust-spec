// `String.startsWith` / `String.endsWith` on the native string type.
pub fn Test_Spec_Assertions_String__startsWith() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(std::rc::Rc::new(
        |prefix, text| {
            let prefix = prefix.unwrap_string();
            let text = text.unwrap_string();
            crate::mk_bool(text.starts_with(&prefix))
        },
    )))
}

pub fn Test_Spec_Assertions_String__endsWith() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(std::rc::Rc::new(
        |suffix, text| {
            let suffix = suffix.unwrap_string();
            let text = text.unwrap_string();
            crate::mk_bool(text.ends_with(&suffix))
        },
    )))
}
