use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::os::raw::c_int;

#[derive(Debug)]
pub struct JumpError {
    jumped_value: c_int,
}

impl JumpError {
    #[inline]
    pub fn new(jumped_value: c_int) -> Self {
        Self { jumped_value }
    }

    #[inline]
    pub fn jumped_value(&self) -> c_int {
        self.jumped_value
    }
}

impl StdError for JumpError {}

impl Display for JumpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jump({})", self.jumped_value)
    }
}
