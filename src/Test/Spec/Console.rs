use std::io::Write;
use std::rc::Rc;

use crate::UnknownType;

pub fn Test_Spec_Console_write(message: String) -> UnknownType {
    Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| {
        let message = purust_core::purust_string_to_utf8_lossy(&message);
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.write_all(message.as_bytes());
        let _ = stdout.flush();
        Value::Unit
    })))
}
