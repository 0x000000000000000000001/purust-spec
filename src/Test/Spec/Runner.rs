// `Test.Spec.Runner.exit :: Int -> Effect Unit`: terminate with the given status.
pub fn Test_Spec_Runner_exit(code: i64) -> crate::UnknownType {
    // Flush the reporters' output before terminating the process.
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
    std::process::exit(code as i32);
}
