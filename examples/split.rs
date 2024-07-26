use bevy::{
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
    RepeatStrategy,
};

fn main() {
    // Define window
    let primary_window = Window {
        title: "Split Screen".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..default()
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
pub fn initialize_camera_system(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    let left_camera = commands
        .spawn(Camera2dBundle {
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
            camera: Camera {
                order: 1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(1280 / 2, 0),
                    physical_size: UVec2::new(1280 / 2, 720),
                    ..default()
                }),
                clear_color: ClearColorConfig::None,
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
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_back.png".to_string(),
                tile_size: UVec2::new(96, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.6),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: UVec2::new(144, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.1),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_front.png".to_string(),
                tile_size: UVec2::new(272, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 2.0,
                ..default()
            },
        ],
        camera: left_camera,
    });
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                path: "sky-stars.png".to_string(),
                tile_size: UVec2::new(53, 55),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.0),
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.5),
                path: "sky-clouds.png".to_string(),
                tile_size: UVec2::new(109, 43),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.0),
                z: 1.0,
                ..default()
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
            right: KeyCode::ArrowRight,
            left: KeyCode::ArrowLeft,
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
        }
    }

    pub fn awsd() -> Self {
        Self {
            right: KeyCode::KeyD,
            left: KeyCode::KeyA,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
        }
    }
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<(Entity, &InputMap), With<Camera>>,
) {
    for (camera, input_map) in camera_query.iter() {
        if keyboard_input.pressed(input_map.right) {
            move_event_writer.send(ParallaxMoveEvent {
                translation: Vec2::new(9.0, 0.0),
                rotation: 0.,
                camera: camera,
            });
        } else if keyboard_input.pressed(input_map.left) {
            move_event_writer.send(ParallaxMoveEvent {
                translation: Vec2::new(-9.0, 0.0),
                rotation: 0.,
                camera: camera,
            });
        }
        if keyboard_input.pressed(input_map.up) {
            move_event_writer.send(ParallaxMoveEvent {
                translation: Vec2::new(0.0, 9.0),
                rotation: 0.,
                camera: camera,
            });
        } else if keyboard_input.pressed(input_map.down) {
            move_event_writer.send(ParallaxMoveEvent {
                translation: Vec2::new(0.0, -9.0),
                rotation: 0.,
                camera: camera,
            });
        }
    }
}
