/// A range of values
pub struct Range {}

/// Represents a variable value
pub struct Variable {
    pub value: f64,
}

impl Variable {
    pub fn new(value: f64) -> Variable {
        Self { value }
    }
}
