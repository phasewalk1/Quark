use crate::prelude::Equation;
use crate::var::Variable;

/// Active screen
pub enum Screen {
    Home,
    Exiting,
    EquationEditor,
    Graph,
}

/// App state
pub struct App<E: Equation> {
    pub screen: Screen,
    pub equation: Option<E>,
    pub variable: Option<Variable>,
}

impl<E: Equation> Default for App<E> {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            equation: None,
            variable: None,
        }
    }
}

impl<E: Equation> App<E> {
    pub fn set_equation(&mut self, equation: E) {
        self.equation = Some(equation);
    }
}
