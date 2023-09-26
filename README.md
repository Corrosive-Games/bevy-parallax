# bevy-parallax

A parallax plugin for the [Bevy Engine](https://bevyengine.org/). This plugin allows you to easily create scrolling parallax backgrounds for your games.

![cyberpunk](assets/cyberpunk.gif)

![fishy](assets/fishy.gif)

## Usage

```rust,no_run
use bevy::prelude::*;
use bevy_parallax::{
    LayerSpeed, LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, CreateParallaxEvent
};

fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
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

pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>
) {
    let camera = commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent::default())
        .id();
    let event = CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "back.png".to_string(),
                tile_size: Vec2::new(96.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.6),
                path: "middle.png".to_string(),
                tile_size: Vec2::new(144.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.1),
                path: "front.png".to_string(),
                tile_size: Vec2::new(272.0, 160.0),
                cols: 1,
                rows: 1,
                scale: 4.5,
                z: 2.0,
                ..default()
            },
        ],
        camera: camera,
    };
    create_parallax.send(event);
}

pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.get_single().unwrap();
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(3.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(-3.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    }
}
```

## Compatible Bevy versions

Compatibility of `bevy-parallax` versions:

| Bevy version | `bevy-parallax` version     |
|:-------------|:----------------------------|
| `0.13`       | `0.8`                       |
| `0.12`       | `0.7`                       |
| `0.11`       | `0.5` - `0.6`               |
| `0.10`       | `0.4`                       |


## Credits

- [Fish World Pack](https://spicylobster.itch.io/fish-world-pack)

- [Warped City 2](https://ansimuz.itch.io/warped-city-2)

- [Mills](https://www.freepik.com/free-vector/flat-wheat-background-with-field_1599667.htm#query=mill%20background%20flat&position=25&from_view=search&track=ais#position=25&query=mill%20background%20flat)
