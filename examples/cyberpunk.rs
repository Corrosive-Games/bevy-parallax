use bevy::prelude::*;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy,
};

fn main() {
    // Define window
    let primary_window = Window {
        title: "Cyberpunk".to_string(),
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
        .insert_resource(ClearColor(Color::rgb_u8(42, 0, 63)))
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
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Mirror),
                path: "cyberpunk_back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.6, 0.6),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 1.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.1),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Mirror),
                path: "cyberpunk_front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 2.0,
                ..Default::default()
            },
        ],
        camera: camera,
    })
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.get_single().unwrap();
    let mut speed = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        speed += Vec2::new(3.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        speed += Vec2::new(-3.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        speed += Vec2::new(0.0, 3.0);
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        speed += Vec2::new(0.0, -3.0);
    }
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: speed,
        camera: camera,
    });
}
