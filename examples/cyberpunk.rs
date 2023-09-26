use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallax::{
    CameraFollow, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxPlugin, ParallaxSystems, RepeatStrategy,
};

#[derive(Component)]
pub struct Player {
    lin_speed: f32,
    ang_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            lin_speed: 900.,
            ang_speed: 3.,
        }
    }
}

fn main() {
    // Define window
    let primary_window = Window {
        title: "Cyberpunk".to_string(),
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
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, initialize_camera_system)
        .add_systems(Update, move_player_system.before(ParallaxSystems))
        .insert_resource(ClearColor(Color::rgb_u8(42, 0, 63)))
        .run();
}

pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    let mut rotation: f32 = 0.;
    let mut direction = Vec2::ZERO;
    for (mut player_transform, player) in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec2::new(1., 0.);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec2::new(-1., 0.)
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec2::new(0., 1.);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec2::new(0., -1.)
        }
        if keyboard_input.pressed(KeyCode::E) {
            rotation -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            rotation += 1.;
        }
        direction = direction.normalize_or_zero() * player.lin_speed * time.delta_seconds();
        rotation = rotation * player.ang_speed * time.delta_seconds();
        player_transform.translation += direction.extend(0.);
        player_transform.rotate_z(rotation);
    }
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let player = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(3.)),
                ..default()
            },
            Player::default(),
        ))
        .id();
    let camera = commands
        .spawn(Camera2dBundle::default())
        .insert(CameraFollow::fixed(player))
        .insert(ParallaxCameraComponent::default())
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.7, 0.85),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                scale: Vec2::splat(4.5),
                z: 0.5,
                flip: (true, false),
                position: Vec2::new(0., 48.),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.6, 0.8),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                scale: Vec2::splat(4.5),
                z: 1.0,
                position: Vec2::new(0., -64.),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.3),
                repeat: LayerRepeat::both(RepeatStrategy::MirrorHorizontally),
                path: "cyberpunk_front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(4.5),
                z: 2.0,
                ..default()
            },
        ],
        camera: camera,
    });
}
