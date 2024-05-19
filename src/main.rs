mod primitives;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_light, spawn_hypercube))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        camera: Camera {
            clear_color: Color::WHITE.into(),
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            radius: 10.0,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 1.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_hypercube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut hypercube = primitives::HyperCube::new();
    hypercube.rotate(primitives::Rotation::XW, 800.0);
    hypercube.rotate(primitives::Rotation::YW, 45.0);
    hypercube.rotate(primitives::Rotation::ZW, 45.0);
    //hypercube.translate(-5.0);
    let indices = vec![
        6, 2, 1, 5, 6, 1, 8, 4, 7, 7, 4, 3, 7, 6, 5, 8, 7, 5, 3, 1, 2, 1, 3, 4, 3, 2, 6, 7, 3, 6,
        8, 5, 4, 1, 4, 5, 5, 1, 13, 9, 13, 1, 8, 16, 4, 12, 4, 16, 8, 4, 5, 1, 5, 4, 16, 13, 12, 9,
        12, 13, 8, 5, 13, 16, 8, 13, 4, 9, 1, 12, 9, 4, 13, 10, 9, 14, 10, 13, 16, 12, 11, 15, 16,
        11, 16, 13, 12, 12, 13, 9, 11, 10, 14, 15, 11, 14, 12, 9, 10, 11, 12, 10, 16, 14, 13, 15,
        14, 16, 14, 10, 2, 6, 14, 2, 11, 15, 3, 7, 3, 15, 11, 3, 10, 3, 2, 10, 15, 14, 7, 7, 14, 6,
        15, 11, 14, 10, 14, 11, 7, 6, 3, 2, 3, 6, 14, 6, 13, 13, 6, 5, 15, 16, 7, 8, 7, 16, 15, 7,
        14, 7, 6, 14, 8, 13, 5, 16, 13, 8, 7, 5, 6, 8, 5, 7, 15, 14, 13, 16, 15, 13, 10, 2, 9, 1,
        9, 2, 11, 12, 3, 4, 3, 12, 11, 3, 10, 2, 10, 3, 12, 9, 4, 4, 9, 1, 12, 10, 9, 11, 10, 12,
        3, 1, 2, 4, 1, 3,
    ];
    let mut normal_indices = vec![];
    for i in indices {
        normal_indices.push(i - 1);
    }
    let projected_vertices: Vec<Vec3> = hypercube.projected_vertices();
    println!("{:?}", projected_vertices);
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, projected_vertices)
    .with_inserted_indices(Indices::U32(normal_indices));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::RED),
            ..default()
        });
}

