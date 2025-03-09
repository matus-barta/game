use bevy::{color::palettes::css::GREEN, prelude::*};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_light, spawn_floor, spawn_house, spawn_placable),
        );
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        Name::new("Sun"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(15.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Floor"),
    );

    commands.spawn(player);
}

fn spawn_house(mut commands: Commands, asset_server: Res<AssetServer>) {
    let house =
        (
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("http://127.0.0.1:3000/assets/house.glb"),
            )),
            Transform::from_xyz(8.5, -1.0, -5.5),
            Name::new("House"),
        );

    commands.spawn(house);
}

fn spawn_placable(mut commands: Commands, asset_server: Res<AssetServer>) {
    let placable = (
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("placable2.glb"))),
        Transform::from_xyz(0.0, 0.0, -1.5),
        Name::new("Placable2"),
    );

    commands.spawn(placable);
}
