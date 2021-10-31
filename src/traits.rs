use std::fmt::{Debug, Display};

/// A custom markdown element.
pub trait Element: Display + Debug {}
