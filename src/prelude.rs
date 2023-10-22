use crate::var::Variable;

/// An interface for equation expressions
pub trait Equation
where
    Self: Clone + core::fmt::Display,
{
    /// Evalutate the equation at a given point
    fn eval(&self, var: &Variable) -> f64;
}
