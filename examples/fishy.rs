use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};
use ron::de::from_bytes;

fn main() {
    let window = WindowDescriptor {
        title: "Fishy".to_string(),
        width: 1280.0,
        height: 720.0,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .insert_resource(ParallaxResource {
            layer_data: from_bytes::<Vec<LayerData>>(include_bytes!(
                "../data/fishy_layer_data.ron"
            ))
            .unwrap(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ParallaxPlugin)
        .add_startup_system(initialize_camera_system)
        .add_system(move_camera_system)
        .run();
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);
}

pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: 3.0,
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: -3.0,
        });
    }
}
