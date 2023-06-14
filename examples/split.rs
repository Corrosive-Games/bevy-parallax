use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent,
    ParallaxPlugin, ParallaxSystems,
};

fn main() {
    // Define window
    let primary_window = Window {
        title: "Split Screen".to_string(),
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
        .add_plugin(ParallaxPlugin)
        .add_startup_system(initialize_camera_system)
        .add_system(move_camera_system.before(ParallaxSystems))
        .run();
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let left_camera = commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Default,
            },
            camera: Camera {
                order: 0,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(1280 / 2, 720),
                    ..default()
                }),
                ..default()
            },
            ..default()
        })
        .insert(ParallaxCameraComponent::new(1))
        .insert(RenderLayers::from_layers(&[0, 1]))
        .insert(InputMap::awsd())
        .id();
    let right_camera = commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(1280 / 2, 0),
                    physical_size: UVec2::new(1280 / 2, 720),
                    ..default()
                }),
                ..default()
            },
            ..default()
        })
        .insert(ParallaxCameraComponent::new(2))
        .insert(RenderLayers::from_layers(&[0, 2]))
        .insert(InputMap::arrows())
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "cyberpunk_back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.6),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 1.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.1),
                path: "cyberpunk_front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 2.0,
                ..Default::default()
            },
        ],
        camera: left_camera,
    });
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                path: "sky-stars.png".to_string(),
                tile_size: Vec2::new(53.0, 55.0),
                cols: 1,
                rows: 1,
                scale: 3.0,
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.5),
                path: "sky-clouds.png".to_string(),
                tile_size: Vec2::new(109.0, 43.0),
                cols: 1,
                rows: 1,
                scale: 4.0,
                z: 1.0,
                ..Default::default()
            },
        ],
        camera: right_camera,
    });
}

#[derive(Component)]
pub struct InputMap {
    right: KeyCode,
    left: KeyCode,
    up: KeyCode,
    down: KeyCode,
}

impl InputMap {
    pub fn arrows() -> Self {
        Self {
            right: KeyCode::Right,
            left: KeyCode::Left,
            up: KeyCode::Up,
            down: KeyCode::Down,
        }
    }

    pub fn awsd() -> Self {
        Self {
            right: KeyCode::D,
            left: KeyCode::A,
            up: KeyCode::W,
            down: KeyCode::S,
        }
    }
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<(Entity, &InputMap), With<Camera>>,
) {
    for (camera, input_map) in camera_query.iter() {
        if keyboard_input.pressed(input_map.right) {
            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: Vec2::new(9.0, 0.0),
                camera: camera,
            });
        } else if keyboard_input.pressed(input_map.left) {
            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: Vec2::new(-9.0, 0.0),
                camera: camera,
            });
        }
        if keyboard_input.pressed(input_map.up) {
            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: Vec2::new(0.0, 9.0),
                camera: camera,
            });
        } else if keyboard_input.pressed(input_map.down) {
            move_event_writer.send(ParallaxMoveEvent {
                camera_move_speed: Vec2::new(0.0, -9.0),
                camera: camera,
            });
        }
    }
}
