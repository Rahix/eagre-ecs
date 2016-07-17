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
/// });
/// ```
#[derive(Debug, Clone, Copy)]
pub struct All;

#[cfg(test)]
mod tests {
    use ::prelude::*;

    #[derive(Debug, Clone)]
    struct Comp1;

    #[derive(Debug, Clone)]
    struct Comp2;

    #[derive(Debug, Clone)]
    struct Comp3;

    #[test]
    fn all_returns_all() {
        let mut system = System::new();
        let mut val = 100;
        for i in 0..val {
            let ent = system.new_entity();
            match i%3 {
                0 => system.add::<Comp1>(ent, Comp1).unwrap(),
                1 => system.add::<Comp2>(ent, Comp2).unwrap(),
                2 => system.add::<Comp3>(ent, Comp3).unwrap(),
                _ => unreachable!(),
            }
        }
        system.run::<::All, _>(|_: &System, _: Entity| {
            val -= 1;
        }).unwrap();
        assert_eq!(0, val);
    }
}
