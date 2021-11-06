use std::fmt::{Debug, Display};

/// A custom markdown element.
pub trait Element<'a>: Display + Debug {
    /// Returns the list of translatable texts of the element.
    fn texts(&self) -> Vec<&'a str>;
}
