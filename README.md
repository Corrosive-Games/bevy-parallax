# bevy-parallax

A parallax plugin for the [Bevy Engine](https://bevyengine.org/). This plugin allows you to easily create scrolling parallax backgrounds for your games.

![cyberpunk](assets/cyberpunk.gif)

![fishy](assets/fishy.gif)

## Usage

```rust
use bevy::prelude::*;
use bevy_parallax::{
    LayerSpeed, LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};

fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: LayerSpeed::Horizontal(0.9),
                    path: "back.png".to_string(),
                    tile_size: Vec2::new(96.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: LayerSpeed::Horizontal(0.6),
                    path: "middle.png".to_string(),
                    tile_size: Vec2::new(144.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 1.0,
                    ..Default::default()
                },
                LayerData {
                    speed: LayerSpeed::Horizontal(0.1),
                    path: "front.png".to_string(),
                    tile_size: Vec2::new(272.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 2.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
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
        .add_system(move_camera_system)
        .run();
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(ParallaxCameraComponent);
}

pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(3.0, 0.0),
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(-3.0, 0.0),
        });
    }
}
```

## Credits
- [Fish World Pack](https://spicylobster.itch.io/fish-world-pack)
- [Warped City 2](https://ansimuz.itch.io/warped-city-2)
