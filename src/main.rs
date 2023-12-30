use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

const STAR_LIMIT: usize = 5000;

#[derive(Component)]
struct Star {
    speed: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, star_spawner)
        .add_systems(Update, star_mover)
        .add_systems(Update, star_deleter)
        .run();
}

fn star_spawner(
    mut commands: Commands,
    stars: Query<&Star>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_stars = stars.iter().count();
    let width = 1000.0;
    let height = 1000.0;
    let far = 900.0;
    let size_scaling = 1.0;

    if num_stars < STAR_LIMIT {
        let to_spawn = STAR_LIMIT - num_stars;
        println!("Need to spawn {to_spawn}");
        for _ in 0..to_spawn {
            let speed = rand::random::<f32>() * 10.0;
            let x = rand::random::<f32>() * width - (width / 2.0);
            let y = rand::random::<f32>() * height - (height / 2.0);
            let z = rand::random::<f32>() * 20.0 + far;
            let size = rand::random::<f32>() * size_scaling;
            let material = materials.add(StandardMaterial {
                emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
                ..default()
            });
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size })),
                    material, //: materials.add(Color::rgb_u8(255, 255, 255).into()),
                    transform: Transform::from_xyz(x, y, -z),
                    ..default()
                },
                Star { speed },
            ));
        }
    }
}

fn star_mover(mut stars: Query<(&mut Star, &mut Transform)>) {
    for (star, mut transform) in stars.iter_mut() {
        transform.translation.z += star.speed;
    }
}

fn star_deleter(mut commands: Commands, stars: Query<(Entity, &Star, &Transform)>) {
    for (entity, _star, transform) in stars.iter() {
        if transform.translation.z > 1.0 {
            // Let's delete it
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
    ));
}
