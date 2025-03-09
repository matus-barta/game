use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_web_asset::WebAssetPlugin;

mod camera;
mod player;
mod web_assets;
mod world;

use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::web_assets::WebAssetsPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            WebAssetPlugin::default(),
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin,
            WebAssetsPlugin,
        ))
        .run();
}
