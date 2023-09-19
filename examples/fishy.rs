use bevy::prelude::*;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin,
    ParallaxSystems,
};
use ron::de::from_bytes;

fn main() {
    // Define window
    let primary_window = Window {
        title: "Fishy".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                // Use nearest filtering so our pixel art renders clear
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ParallaxPlugin)
        .add_systems(Startup, initialize_camera_system)
        .add_systems(Update, move_camera_system.before(ParallaxSystems))
        .run();
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let camera = commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: from_bytes::<Vec<LayerData>>(include_bytes!("../data/fishy_layer_data.ron"))
            .unwrap(),
        camera: camera,
    });
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<Entity, With<Camera>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    let camera = camera_query.get_single().unwrap();
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(3.0, 0.0),
            camera: camera,
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(-3.0, 0.0),
            camera: camera,
        });
    }
}
