use ecs::entity_manager::EntityManager;
use ecs::system::System;

#[derive(Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
struct Velocity {
    pub x: f32,
    pub y: f32,
}

fn update_position((pos, vel): (&mut Position, &Velocity)) {
    pos.x += vel.x;
    pos.y += vel.y;
}

fn main() {
    let mut entity_manager = EntityManager::new();
    entity_manager.new_entity((Position { x: 0.1, y: 0.0 },));
    entity_manager.new_entity((Position { x: 0.2, y: 0.0 },));
    entity_manager.new_entity((Position { x: 0.3, y: 0.0 },));
    entity_manager.new_entity((Position { x: 0.0, y: 0.1 }, Velocity { x: 1.0, y: 1.0 }));
    entity_manager.new_entity((Position { x: 0.0, y: 0.2 }, Velocity { x: 1.0, y: 1.0 }));
    entity_manager.new_entity((Position { x: 0.0, y: 0.3 }, Velocity { x: 1.0, y: 1.0 }));

    let mut log_position = System::<(&Position,)>::new(|(pos,)| {
        println!("{:#?}", pos);
    });
    let mut update_position_system = System::<(&mut Position, &Velocity)>::new(|(pos, vel)| {
        pos.x += vel.x;
        pos.y += vel.y;
    });

    log_position.execute(&mut entity_manager);
    update_position_system.execute(&mut entity_manager);
    log_position.execute(&mut entity_manager);
}
