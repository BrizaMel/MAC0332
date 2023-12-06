pub struct TerminalExpression {
    pub expression: String,
}

impl TerminalExpression {
    pub fn new(expression:String) -> Self {
        Self {expression}
    }
}