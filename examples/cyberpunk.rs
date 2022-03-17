use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_parallax::{LayerData, ParallaxPlugin, ParallaxResource};

fn main() {
    let window = WindowDescriptor {
        title: "Cyberpunk".to_string(),
        width: 1280.0,
        height: 720.0,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .insert_resource(ParallaxResource::new(vec![
            LayerData {
                speed: 1.0,
                path: "back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 0.0,
            },
            LayerData {
                speed: 1.0,
                path: "middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 1.0,
            },
            LayerData {
                speed: 1.0,
                path: "front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 2.0,
            },
        ]))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ParallaxPlugin)
        .add_startup_system(initialize_camera_system)
        .run();
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
