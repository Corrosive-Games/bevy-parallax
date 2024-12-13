use crate::layer;
use bevy::{prelude::*, render::view::RenderLayers};

#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::prelude::*;

/// Event to setup and create parallax
#[derive(Event, Debug)]
pub struct CreateParallaxEvent {
    pub layers_data: Vec<layer::LayerData>,
    pub camera: Entity,
}

impl CreateParallaxEvent {
    /// Create layers from layer data
    pub fn create_layers(
        &self,
        commands: &mut Commands,
        window_size: Vec2,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
        render_layer: u8,
    ) {
        // Spawn new layers using layer_data
        for (i, layer) in self.layers_data.iter().enumerate() {
            let texture: Handle<Image> = asset_server.load(&layer.path);
            let texture_atlas = layer.create_texture_atlas_layout();
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let sprite_bundle = layer.create_sprite(texture, TextureAtlas::from(texture_atlas_handle));

            // Spawn a grid of textures, so that they convincingly wrap around the screen when scrolling.
            // For no repeat layers, only spawn a single row or column in direction of their movement.

            // In every row of the grid, our goal is to have a central texture and at least two that surround it,
            // plus as much as it would take to fill the rest of the window space in both directions.
            // The grid should have a pair number so the mirror repeat can work correctly
            // Same logic applies to vertical placement.

            let max_length = window_size.length();

            let y_max_index = match layer.repeat.has_vertical() {
                true => f32::ceil(max_length / (layer.tile_size.y as f32 * layer.scale.y)) as i32,
                false => 0,
            };

            let x_max_index = match layer.repeat.has_horizontal() {
                true => f32::ceil(max_length / (layer.tile_size.x as f32 * layer.scale.x)) as i32,
                false => 0,
            };

            let texture_count = Vec2::new(f32::max(2.0 * x_max_index as f32, 1.), f32::max(2.0 * y_max_index as f32, 1.));

            let x_range = if layer.repeat.has_horizontal() {
                (-x_max_index + 1)..=x_max_index
            } else {
                0..=0
            };
            let y_range = if layer.repeat.has_vertical() {
                (-y_max_index + 1)..=y_max_index
            } else {
                0..=0
            };

            // Spawn parallax layer entity
            let mut entity_commands = commands.spawn_empty();
            entity_commands
                .insert(Name::new(format!("Parallax Layer ({})", i)))
                .insert(RenderLayers::from_layers(&[render_layer.into()]))
                .insert((Transform {
                    translation: Vec3::new(layer.position.x, layer.position.y, layer.z),
                    scale: layer.scale.extend(1.0),
                    ..default()
                }, Visibility::Visible))
                .with_children(|parent| {
                    for x in x_range {
                        for y in y_range.clone() {
                            let repeat_strategy = layer.repeat.get_strategy();
                            let mut adjusted_sprite = sprite_bundle.clone();
                            repeat_strategy.transform(&mut adjusted_sprite, (x, y));

                            let mut transform = Transform::default();
                            transform.translation.x = layer.tile_size.x as f32 * x as f32;
                            transform.translation.y = layer.tile_size.y as f32 * y as f32;
                            let mut child_commands = parent.spawn((adjusted_sprite, transform));
                            child_commands
                                .insert(RenderLayers::from_layers(&[render_layer.into()]))
                                .insert(layer.crate_layer_texture());
                            if let Some(animation_bundle) = layer.create_animation_bundle() {
                                child_commands.insert(animation_bundle);
                            }
                        }
                    }
                });

            // Add layer component to entity
            entity_commands
                .insert(layer::LayerComponent {
                    speed: match layer.speed {
                        layer::LayerSpeed::Horizontal(vx) => Vec2::new(vx, 0.0),
                        layer::LayerSpeed::Vertical(vy) => Vec2::new(0.0, vy),
                        layer::LayerSpeed::Bidirectional(vx, vy) => Vec2::new(vx, vy),
                    },
                    repeat: layer.repeat.clone(),
                    texture_count,
                    camera: self.camera,
                })
                .insert(RenderLayers::from_layers(&[render_layer.into()]));
        }
    }
}

/// Event used to update parallax
#[derive(Event, Debug)]
pub struct ParallaxMoveEvent {
    /// camera translation
    pub translation: Vec2,

    /// camera rotation
    pub rotation: f32,

    pub camera: Entity,
}

impl ParallaxMoveEvent {
    pub fn has_translation(&self) -> bool {
        self.translation != Vec2::ZERO
    }

    pub fn has_right_translation(&self) -> bool {
        self.translation.x > 0.
    }

    pub fn has_left_translation(&self) -> bool {
        self.translation.x < 0.
    }

    pub fn has_up_translation(&self) -> bool {
        self.translation.y > 0.
    }

    pub fn has_down_translation(&self) -> bool {
        self.translation.y < 0.
    }
}

/// Attach to a single camera to be used with parallax
#[derive(Component)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
pub struct ParallaxCameraComponent {
    pub render_layer: u8,
    pub limits: Vec2Limit,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct Limit {
    pub min: f32,
    pub max: f32,
}

impl Default for Limit {
    fn default() -> Self {
        Self {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}

impl Limit {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn zero_to_infinity() -> Self {
        Self {
            min: 0.,
            max: f32::INFINITY,
        }
    }

    pub fn zero_to(max: f32) -> Self {
        Self { min: 0., max }
    }

    pub fn fix(&self, value: f32) -> f32 {
        f32::min(f32::max(value, self.min), self.max)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct Vec2Limit {
    pub x: Limit,
    pub y: Limit,
}

impl Vec2Limit {
    pub fn new(x: Limit, y: Limit) -> Self {
        Self { x, y }
    }

    pub fn fix(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.x.fix(vec.x), self.y.fix(vec.y))
    }
}

impl Default for Vec2Limit {
    fn default() -> Self {
        Self {
            x: default(),
            y: default(),
        }
    }
}

impl ParallaxCameraComponent {
    pub fn inside_limits(&self, translation: Vec2) -> Vec2 {
        self.limits.fix(translation)
    }

    pub fn new(render_layer: u8) -> Self {
        Self {
            render_layer,
            ..default()
        }
    }
}

impl Default for ParallaxCameraComponent {
    fn default() -> Self {
        Self {
            render_layer: 0,
            limits: default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::ParallaxMoveEvent;

    #[test]
    fn test_check() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_parallax_event() {
        let camera = Entity::from_raw(0);

        let no_movement = ParallaxMoveEvent {
            translation: Vec2::ZERO,
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(no_movement.has_translation(), false);
        assert_eq!(no_movement.has_up_translation(), false);
        assert_eq!(no_movement.has_down_translation(), false);
        assert_eq!(no_movement.has_left_translation(), false);
        assert_eq!(no_movement.has_right_translation(), false);

        let up = ParallaxMoveEvent {
            translation: Vec2::new(0., 1.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(up.has_translation(), true);
        assert_eq!(up.has_up_translation(), true);
        assert_eq!(up.has_down_translation(), false);
        assert_eq!(up.has_left_translation(), false);
        assert_eq!(up.has_right_translation(), false);

        let down = ParallaxMoveEvent {
            translation: Vec2::new(0., -1.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(down.has_translation(), true);
        assert_eq!(down.has_up_translation(), false);
        assert_eq!(down.has_down_translation(), true);
        assert_eq!(down.has_left_translation(), false);
        assert_eq!(down.has_right_translation(), false);

        let left = ParallaxMoveEvent {
            translation: Vec2::new(-1., 0.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(left.has_translation(), true);
        assert_eq!(left.has_up_translation(), false);
        assert_eq!(left.has_down_translation(), false);
        assert_eq!(left.has_left_translation(), true);
        assert_eq!(left.has_right_translation(), false);

        let right = ParallaxMoveEvent {
            translation: Vec2::new(1., 0.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(right.has_translation(), true);
        assert_eq!(right.has_up_translation(), false);
        assert_eq!(right.has_down_translation(), false);
        assert_eq!(right.has_left_translation(), false);
        assert_eq!(right.has_right_translation(), true);

        let left_down = ParallaxMoveEvent {
            translation: Vec2::new(-1., -1.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(left_down.has_translation(), true);
        assert_eq!(left_down.has_up_translation(), false);
        assert_eq!(left_down.has_down_translation(), true);
        assert_eq!(left_down.has_left_translation(), true);
        assert_eq!(left_down.has_right_translation(), false);

        let up_right = ParallaxMoveEvent {
            translation: Vec2::new(1., 1.),
            rotation: 0.,
            camera: camera,
        };
        assert_eq!(up_right.has_translation(), true);
        assert_eq!(up_right.has_up_translation(), true);
        assert_eq!(up_right.has_down_translation(), false);
        assert_eq!(up_right.has_left_translation(), false);
        assert_eq!(up_right.has_right_translation(), true);
    }
}
