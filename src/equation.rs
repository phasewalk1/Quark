use crate::prelude::Equation;
use crate::var::Variable;

#[derive(Clone, Debug)]
pub struct LinearEquation {
    pub m: f64,
    pub b: f64,
}

impl LinearEquation {
    pub fn new(m: f64, b: f64) -> Self {
        Self { m, b }
    }
}

impl Equation for LinearEquation {
    fn eval(&self, var: &Variable) -> f64 {
        self.m * var.value + self.b
    }
}

impl core::fmt::Display for LinearEquation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "y = {}x + {}", self.m, self.b)
    }
}
