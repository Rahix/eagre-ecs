//! eagre-ecs
//! =========
//!
//! An Entity-Component System
//!
//! # Example
//!
//! ```
//! extern crate eagre_ecs;
//! use eagre_ecs::prelude::*;
//!
//! #[derive(Clone, Debug)]
//! struct Foo(i32);
//!
//! #[derive(Clone, Debug)]
//! struct Bar(i32);
//!
//! fn main() {
//!     let mut system = System::new();
//!     for i in 0..100 {
//!         let ent = system.new_entity();
//!         match i%2 {
//!             0 => system.add(ent, Foo(i)).unwrap(),
//!             1 => system.add(ent, Bar(i)).unwrap(),
//!             _ => unreachable!(),
//!         }
//!     }
//!     system.run_mut::<Foo, _>(|sys: &mut System, ent: Entity| {
//!         sys.borrow_mut::<Foo>(ent).unwrap().0 += 1;
//!     }).unwrap();
//!     system.run_mut::<Bar, _>(|sys: &mut System, ent: Entity| {
//!         sys.borrow_mut::<Bar>(ent).unwrap().0 -= 1;
//!     }).unwrap();
//!     system.run::<eagre_ecs::All, _>(|sys: &System, ent: Entity| {
//!         if sys.has::<Foo>(ent) {
//!             println!("Foo: {}", sys.borrow::<Foo>(ent).unwrap().0);
//!         } else {
//!             println!("Bar: {}", sys.borrow::<Bar>(ent).unwrap().0);
//!         }
//!     }).unwrap();
//! }
//! ```
#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations)]

pub mod component;
pub mod entity;
pub mod error;
pub mod system;

pub use component::{Component, All};
pub use entity::Entity;
pub use error::{EcsError, EcsResult};
pub use system::System;

/// For lazy people
pub mod prelude {
    pub use ::{Entity, System};
}
