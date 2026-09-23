// `String.startsWith` / `String.endsWith` on the native string type.
pub fn Test_Spec_Assertions_String__startsWith(prefix: String, text: String) -> bool {
    text.starts_with(&prefix)
}

pub fn Test_Spec_Assertions_String__endsWith(suffix: String, text: String) -> bool {
    text.ends_with(&suffix)
}
