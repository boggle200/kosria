use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};

#[derive(Component)]
pub struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
}

impl Movable {
    pub fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 5.0,
            speed: 2.0,
        }
    }
}

pub fn bevy_main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_sphere)
        .run();
}

// Startup system to setup the scene and spawn all relevant entities.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a cube to visualize translation.
    let entity_spawn = Vec3::ZERO;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(entity_spawn),
            ..default()
        },
        Movable::new(entity_spawn),
    ));

    // Spawn a camera looking at the entities to show what's happening in this example.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(entity_spawn, Vec3::Y),
        ..default()
    });

    // Add a light source for better 3d visibility.
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn move_sphere(mut cubes: Query<(&mut Transform, &mut Movable)>, timer: Res<Time>) {
    for (mut transform, mut cube) in &mut cubes {
        // Check if the entity moved too far from its spawn, if so invert the moving direction.
        if (cube.spawn - transform.translation).length() > cube.max_distance {
            cube.speed *= -1.0;
        }
        let direction = transform.local_x();
        transform.translation += direction * cube.speed * timer.delta_seconds();
    }
}