use bevy::{color::palettes::css::RED, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Name::new("Player"),
    );

    commands.spawn(player);
}
