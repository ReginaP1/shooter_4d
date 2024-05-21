use bevy::prelude::*;
use nalgebra::Vector4;
use primitives::tesseract::Tesseract;
use primitives::Rotation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_light, spawn_hypercube))
        .add_systems(Update, animate_hypercube)
        .run();
}

#[derive(Component)]
struct TesseractComponent {
    object: Tesseract,
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        camera: Camera {
            clear_color: Color::GRAY.into(),
            ..default()
        },
        transform: Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 200000.0,
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
    let mut tesseract = Tesseract::new();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(tesseract.mesh()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(TesseractComponent { object: tesseract });
}

fn animate_hypercube(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut TesseractComponent, &Handle<Mesh>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let camera_transform = camera.get_single_mut().unwrap();
    for (mut tesseract_component, mesh_handle) in query.iter_mut() {
        tesseract_component
            .object
            .rotate(Rotation::XW, 0.9 * time.delta_seconds());
        // tesseract_component
        //     .object
        //     .rotate(Rotation::YW, 0.7 * time.delta_seconds());
        // tesseract_component
        //     .object
        //     .rotate(Rotation::ZW, 0.5 * time.delta_seconds());
        // tesseract_component
        //     .object
        //     .translate(-6.0 * time.delta_seconds());
        let projected_vertices = tesseract_component.object.projected_vertices(
            Vector4::new(5.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            45.0,
        );
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, projected_vertices);
        }
    }
}
