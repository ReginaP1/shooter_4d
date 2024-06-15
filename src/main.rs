use bevy::prelude::*;
use nalgebra::Vector4;
use primitives::tesseract::Tesseract;
use primitives::{Rotation, Axis};

pub struct SpawnObjectsPlugin;

impl Plugin for SpawnObjectsPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(
            Startup,
            (
                spawn_camera, spawn_light, spawn_hypercube, spawn_floor
            )
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpawnObjectsPlugin)
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
        transform: Transform::from_xyz(1.0, 4.0, 1.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 100000.0,
            radius: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::GREEN),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

fn spawn_hypercube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut tesseract = Tesseract::new();
    tesseract.scale(Axis::X, 0.25);
    tesseract.scale(Axis::Y, 2.0);
    tesseract.scale(Axis::Z, 0.25);
    tesseract.scale(Axis::W, 2.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(tesseract.mesh()),
            material: materials.add(Color::CYAN),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(TesseractComponent { object: tesseract });
}

fn animate_hypercube(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut TesseractComponent, &Handle<Mesh>)>,
    // mut camera: Query<&mut Transform, With<Camera>>,
) {
    // let translation = camera.get_single_mut().unwrap().translation;
    for (mut tesseract_component, mesh_handle) in query.iter_mut() {
        tesseract_component
            .object
            .rotate(Rotation::XW, 0.9 * time.delta_seconds());
        // tesseract_component
        //     .object
        //     .rotate(Rotation::YW, 0.7 * time.delta_seconds());
        // tesseract_component
        //     .object
        //     .rotate(Rotation::ZW, 1.0 * time.delta_seconds());
        let projected_vertices = tesseract_component.object.projected_vertices(
            Vector4::new(5.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            45.0,
        );
        let normals = tesseract_component.object.get_normals(&projected_vertices);
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, projected_vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
    }
}
