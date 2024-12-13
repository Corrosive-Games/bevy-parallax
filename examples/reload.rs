use bevy::prelude::*;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems,
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
        .add_systems(Update, (reload_system, despawn_all))
        .run();
}

pub fn new_create_parallax_event(camera: Entity) -> CreateParallaxEvent {
    CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "cyberpunk_back.png".to_string(),
                tile_size: UVec2::new(96, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 0.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.6),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: UVec2::new(144, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 1.0,
                ..Default::default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.1),
                path: "cyberpunk_front.png".to_string(),
                tile_size: UVec2::new(272, 160),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 2.0,
                ..Default::default()
            },
        ],
        camera: camera,
    }
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    let camera = commands
        .spawn(Camera2d::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    create_parallax.send(new_create_parallax_event(camera));
}

pub fn reload_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<Entity, With<Camera>>,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let camera = camera_query.get_single().unwrap();
    if keyboard_input.just_released(KeyCode::KeyR) {
        create_parallax.send(new_create_parallax_event(camera));
    }
}

pub fn despawn_all(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    all_query: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    if keyboard_input.just_released(KeyCode::KeyQ) {
        for entity in all_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.get_single().unwrap();
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(3.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(-3.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    }
}
