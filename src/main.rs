use bevy::prelude::*;

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands
        .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane{ size: 8.0, subdivisions: 0 })),
                material: materials.add(Color::rgb(1.,0.9,0.9).into()),
                transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
                ..Default::default()
    });
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-0.7, 20.0, 4.0),
            )),
            ..Default::default()
    });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1200., 1200.).into(),
                title: "Chess".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}
