use bevy::prelude::*;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallax::{
    Animation, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin,
    ParallaxSystems, RepeatStrategy,
};

fn main() {
    // Define window
    let primary_window = Window {
        title: "Mills".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };

    let mut app = App::new();
    app.add_plugins(
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
    .insert_resource(ClearColor(Color::srgb_u8(156, 219, 248)));
    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    let camera = commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.99, 0.99),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.15),
                z: 0.6,
                position: Vec2::new(0., 50.),
                color: Color::BLACK,
                animation: Some(Animation::FPS(30.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.98, 0.98),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.25),
                z: 0.7,
                position: Vec2::new(0., 50.),
                color: bevy::color::palettes::css::DARK_GRAY.into(),
                index: 2,
                animation: Some(Animation::FPS(28.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.95, 0.95),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.5),
                z: 0.8,
                position: Vec2::new(0., 25.),
                color: bevy::color::palettes::css::GRAY.into(),
                index: 5,
                animation: Some(Animation::FPS(26.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
                path: "mills-back.png".to_string(),
                tile_size: UVec2::new(1123, 794),
                cols: 6,
                rows: 1,
                scale: Vec2::splat(0.8),
                z: 0.9,
                color: Color::WHITE,
                index: 1,
                animation: Some(Animation::FPS(24.)),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.8, 0.8),
                repeat: LayerRepeat::horizontally(RepeatStrategy::MirrorBoth),
                path: "mills-front.png".to_string(),
                tile_size: UVec2::new(750, 434),
                cols: 6,
                rows: 1,
                z: 1.0,
                scale: Vec2::splat(1.5),
                position: Vec2::new(0., -100.),
                index: 3,
                animation: Some(Animation::FPS(20.)),
                ..default()
            },
        ],
        camera: camera,
    });
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera>>,
) {
    let (camera, mut camera_transform) = camera_query.get_single_mut().unwrap();
    let speed = 20.;
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        camera_transform.rotate_z(0.1);
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        camera_transform.rotate_z(-0.1);
    }
    move_event_writer.send(ParallaxMoveEvent {
        translation: direction.normalize_or_zero() * speed,
        camera: camera,
        rotation: 0.,
    });
}
