pub enum Code {
    Good,
    NameErr,
    OutOfRange,
    BadArguments,
    FunctionWithinFunctionErr,
    ClosingOfNonFunctionErr,
    LocalVariableNotInFunction,
}

pub struct Return {
    pub message: String,
    pub code: Code,
}

impl Return {
    pub fn new(message: String, code: Code) -> Self {
        Self { message, code }
    }
}
