[![Build Status](https://travis-ci.org/Rahix/eagre-ecs.svg?branch=master)](https://travis-ci.org/Rahix/eagre-ecs)
eagre-ecs
=========

An Entity-Component System

## Installation ##

Add the following line to your `Cargo.toml`:

```toml
eagre-ecs = { git = "https://github.com/Rahix/eagre-ecs" }
```

## Example ##

```rust
extern crate eagre_ecs;
use eagre_ecs::prelude::*;

#[derive(Clone, Debug)]
struct Foo(i32);

#[derive(Clone, Debug)]
struct Bar(i32);

fn main() {
    let mut system = System::new();
    for i in 0..100 {
        let ent = system.new_entity();
        match i%2 {
            0 => system.add(ent, Foo(i)).unwrap(),
            1 => system.add(ent, Bar(i)).unwrap(),
            _ => unreachable!(),
        }
    }
    system.run_mut::<Foo, _>(|sys: &mut System, ent: Entity| {
        sys.borrow_mut::<Foo>(ent).unwrap().0 += 1;
    }).unwrap();
    system.run_mut::<Bar, _>(|sys: &mut System, ent: Entity| {
        sys.borrow_mut::<Bar>(ent).unwrap().0 -= 1;
    }).unwrap();
    system.run::<eagre_ecs::All, _>(|sys: &System, ent: Entity| {
        if sys.has::<Foo>(ent) {
            println!("Foo: {}", sys.borrow::<Foo>(ent).unwrap().0);
        } else {
            println!("Bar: {}", sys.borrow::<Bar>(ent).unwrap().0);
        }
    }).unwrap();
}
```

## License ##
eagre-ecs is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
