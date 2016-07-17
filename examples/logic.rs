extern crate rand;
extern crate eagre_ecs;

use rand::Rng;

use eagre_ecs::prelude::*;

#[derive(Clone, Debug)]
struct CanUpdate;

#[derive(Clone, Debug)]
struct Data1 {
    a: i32,
    b: i32,
}

#[derive(Clone, Debug)]
struct Data2 {
    a: f32,
    b: f32,
}


fn main() {
    let mut system = System::new();
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let ent = system.new_entity();
        system.add(ent, CanUpdate).unwrap();
        match rng.gen::<bool>() {
            true => system.add(ent, Data1 {
                a: rng.gen::<i32>() % 100,
                b: rng.gen::<i32>() % 100,
            }).unwrap(),
            false => system.add(ent, Data2 {
                a: rng.gen::<f32>(),
                b: rng.gen::<f32>(),
            }).unwrap(),
        }
    }
    for _ in 0..10 {
        system.run_mut::<CanUpdate, _>(|sys: &mut System, ent: Entity| {
            if sys.has::<Data1>(ent) {
                let mut data = sys.borrow_mut::<Data1>(ent).unwrap();
                let tmp = data.a;
                data.a = data.a - data.b;
                data.b = data.b - tmp;
            } else {
                let mut data = sys.borrow_mut::<Data2>(ent).unwrap();
                let tmp = data.a;
                data.a = data.a - data.b;
                data.b = data.b - tmp;
            }
        }).unwrap();
    }
    system.run::<Data1, _>(|sys: &System, ent: Entity| {
        let data = sys.borrow::<Data1>(ent).unwrap();
        println!("Entity {} has {:?}", ent, data);
    }).unwrap();
    system.run::<Data2, _>(|sys: &System, ent: Entity| {
        let data = sys.borrow::<Data2>(ent).unwrap();
        println!("Entity {} has {:?}", ent, data);
    }).unwrap();
    system.run_mut::<Data2, _>(|sys: &mut System, ent: Entity| {
        sys.remove_entity(ent).unwrap();
    }).unwrap();
    println!("After removing all Data2 entities, {} entities are left.", system.num_entities());
}
