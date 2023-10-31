use crate::query_representation::initial::Expression;

pub struct OrExpression {
    pub left_expression: String,
    pub right_expression: String
}

impl OrExpression {
    pub fn new(left_expression:String, right_expression:String) -> Self {
        Self {left_expression,right_expression}
    }
}

pub struct AndExpression {
    pub left_expression: String,
    pub right_expression: String
}

impl AndExpression {
    pub fn new(left_expression:String, right_expression:String) -> Self {
        Self {left_expression,right_expression}
    }
}