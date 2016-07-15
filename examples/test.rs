extern crate eagre_ecs;

use eagre_ecs::*;

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
    let system = System::new();

    // Entity 1
    let entity = Entity::new(&system);
    entity.add(MyComponent { x: 12, y: 42 });
    entity.add(MySecondComponent { name: "Foo".to_string() });

    // Entity 2
    let entity2 = Entity::new(&system);
    entity2.add(MyComponent { x: 13, y: 16 });

    // Entity 3
    let entity3 = Entity::new(&system);
    entity3.add(MySecondComponent { name: "Bar".to_string() });

    // Do something
    println!("All Entities having a MyComponent:");
    system.run::<MyComponent>(&|ent| {
        let x = 0;
        let y = 0;
        println!("Entity {} is at ({},{})", ent.id, x, y);
    });
    println!("All Entities having a MySecondComponent:");
    system.run::<MySecondComponent>(&|ent| {
        let name = "<none>".to_string();
        println!("Entity {} is called {}", ent.id, name);
    })
}
