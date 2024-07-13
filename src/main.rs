use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));
}

const MOVE_SPEED: f32 = 6.0;

fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }
    }
}
