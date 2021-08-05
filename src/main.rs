use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_startup_system(spawn_hive.system())
        .add_startup_system(spawn_flowers.system())
        .add_system(hello_world.system())
        .run();
}

// Structs

enum BeeState {
    Idle,
    Foraging(Position),
    Returning(Position)
}

struct Bee(BeeState);

struct Flower { polinated: bool }

struct Position { x: u32, y: u32 }

struct Hive { last_bee_generated: u64 }

struct Food(u32);

struct Energy(u32);

// Spawners

fn spawn_hive(mut commands: Commands) {
    commands.spawn()
        .insert(Hive { last_bee_generated: 0 })
        .insert(Food(100))
        .insert(Position { x: 30, y: 30 });

    for _ in 1..20 {
        commands.spawn()
            .insert(Bee(BeeState::Idle))
            .insert(Food(0))
            .insert(Energy(20))
            .insert(Position { x: 30, y: 30 });
    }
}

fn spawn_flowers(mut commands: Commands) {
    for x in 29..31 {
        for y in 29..31 {
            if y != 30 && x != 30 {
                commands.spawn()
                    .insert(Flower { polinated: false })
                    .insert(Food(15))
                    .insert(Position { x, y });
            }
        }
    }
}

// Systems


fn hello_world(time: Res<Time>) {
    println!("hello time {:?}", time);
}
