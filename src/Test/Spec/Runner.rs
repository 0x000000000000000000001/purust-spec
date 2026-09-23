// `Test.Spec.Runner.exit`: terminate with the given status.
pub fn Test_Spec_Runner_exit() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(|code| {
        let code = code.unwrap_int();
        std::process::exit(code as i32);
    })))
}
