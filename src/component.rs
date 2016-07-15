use std::any;
use std::fmt;

pub trait Component: any::Any + fmt::Debug { }

impl<T: any::Any + fmt::Debug> Component for T { }

#[cfg(test)]
mod tests {
    use super::*;
}
