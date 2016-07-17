extern crate eagre_ecs;

use eagre_ecs::prelude::*;

#[derive(Debug, Clone)]
struct MyComponent {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct MySecondComponent {
    name: String,
}

fn main() {
    let mut system = System::new();

    // Entity 1
    let entity = system.new_entity();
    system.add(entity, MyComponent { x: 12, y: 42 });
    system.add(entity, MySecondComponent { name: "Foo".to_string() });

    // Entity 2
    let entity = system.new_entity();
    system.add(entity, MyComponent { x: 13, y: 16 });

    // Entity 3
    let entity = system.new_entity();
    system.add(entity, MySecondComponent { name: "Bar".to_string() });

    // Do something
    println!("All Entities having a MyComponent:");
    system.run::<MyComponent, _>(|sys: &System, ent: Entity| {
        let x = sys.borrow::<MyComponent>(ent).unwrap().x;
        let y = sys.borrow::<MyComponent>(ent).unwrap().y;
        println!("Entity {} is at ({},{})", ent, x, y);
    }).unwrap();
    println!("All Entities having a MySecondComponent:");
    system.run::<MySecondComponent, _>(|sys: &System, ent: Entity| {
        let name = sys.get::<MySecondComponent>(ent).unwrap().name;
        println!("Entity {} is called {}", ent, name);
    }).unwrap();
    println!("Change some values");
    system.run_mut::<MyComponent, _>(|sys: &mut System, ent: Entity| {
        sys.borrow_mut::<MyComponent>(ent).unwrap().x += 1;
        sys.borrow_mut::<MyComponent>(ent).unwrap().y += 1;
    }).unwrap();
    println!("All Entities having a MyComponent(again):");
    system.run::<MyComponent, _>(|sys: &System, ent: Entity| {
        let x = sys.borrow::<MyComponent>(ent).unwrap().x;
        let y = sys.borrow::<MyComponent>(ent).unwrap().y;
        println!("Entity {} is at ({},{})", ent, x, y);
    }).unwrap();
}
