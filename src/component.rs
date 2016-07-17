//! Components related types and traits
use std::any;
use std::fmt;

/// Trait for components
///
/// Implemented by everything
pub trait Component: any::Any + fmt::Debug + Clone { }

impl<T: any::Any + fmt::Debug + Clone> Component for T { }

/// Empty Component, that every entity has
///
/// Used for iterating over all entities
///
/// ```
/// # use eagre_ecs::prelude::*;
///
/// let mut system = System::new();
/// for _ in 0..100 {
///     system.new_entity();
/// }
/// system.run::<eagre_ecs::All, _>(|sys: &System, ent: Entity| {
///     println!("Entity: {}", ent);
/// })
/// ```
#[derive(Debug, Clone, Copy)]
pub struct All;

#[cfg(test)]
mod tests {
    use super::*;
}
