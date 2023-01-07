use bevy::prelude::*;

#[derive(Component)] // First Component, an empty struct that will be used to put a tag on an
struct SnakeHead;    // entity.
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_startup_system(snake_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
// Commands are used to perform impactful changes to the world.
// I.e spawning or despawning entities, adding or removing components, etc.


fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default() // SpriteBundle has a lot of fields, so we use the default() method to fill
                        // them in with the default values.
        })
        .insert(SnakeHead);
} // This function (system) spawns a new entity which uses something called a SpriteBundle to
  // contain all of the components. the SpriteBundle contains Sprite and Transform.

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform,  With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
    }
}

