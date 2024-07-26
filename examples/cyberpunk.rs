use bevy::prelude::*;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_parallax::{
    CameraFollow, CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, Limit, ParallaxCameraComponent, ParallaxPlugin, ParallaxSystems,
    RepeatStrategy, Vec2Limit, PID,
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
    .add_systems(Update, move_player_system.before(ParallaxSystems))
    .insert_resource(ClearColor(Color::srgb_u8(42, 0, 63)));
    #[cfg(feature = "bevy-inspector-egui")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.run();
}

pub fn move_player_system(keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player_query: Query<(&mut Transform, &Player)>) {
    let mut rotation: f32 = 0.;
    let mut direction = Vec2::ZERO;
    for (mut player_transform, player) in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec2::new(1., 0.);
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec2::new(-1., 0.)
        }
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec2::new(0., 1.);
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec2::new(0., -1.)
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            rotation -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            rotation += 1.;
        }
        direction = direction.normalize_or_zero() * player.lin_speed * time.delta_seconds();
        rotation = rotation * player.ang_speed * time.delta_seconds();
        player_transform.translation += direction.extend(0.);
        player_transform.rotate_z(rotation);
    }
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    let player = commands
        .spawn((
            Name::new("Player"),
            SpriteBundle {
                sprite: Sprite {
                    color: bevy::color::palettes::css::YELLOW.into(),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(3.)),
                ..default()
            },
            Player::default(),
        ))
        .id();
    let y_limit = Limit::zero_to(500.);
    let x_pid = PID::new(0.1, 0.5, 0.01);
    let y_pid = x_pid.with_integral_limit(Limit::new(-25., 25.));
    let offset = Vec2::new(200., 0.);
    let camera = commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(offset.extend(0.)),
            ..default()
        })
        //.insert(CameraFollow::fixed(player).with_offset(offset))
        //.insert(CameraFollow::proportional(player, 0.1).with_offset(offset))
        .insert(CameraFollow::pid_xyz(player, &x_pid, &y_pid, &x_pid).with_offset(offset))
        .insert(ParallaxCameraComponent {
            limits: Vec2Limit::new(Limit::default(), y_limit),
            ..default()
        })
        .id();
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
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
                speed: LayerSpeed::Bidirectional(0.7, 0.85),
                repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
                path: "cyberpunk_middle.png".to_string(),
                tile_size: UVec2::new(144, 160),
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
                tile_size: UVec2::new(144, 160),
                scale: Vec2::splat(4.5),
                z: 1.0,
                position: Vec2::new(0., -64.),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.1, 0.3),
                repeat: LayerRepeat::both(RepeatStrategy::MirrorHorizontally),
                path: "cyberpunk_front.png".to_string(),
                tile_size: UVec2::new(272, 160),
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
