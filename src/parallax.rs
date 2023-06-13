use std::cmp::max;

use crate::layer;
use bevy::{prelude::*, render::view::RenderLayers};

/// Event to setup and create parallax
#[derive(Debug)]
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
        texture_atlases: &mut Assets<TextureAtlas>,
        render_layer: u8,
    ) -> Vec<Entity> {
        let mut entities = vec![];
        // Spawn new layers using layer_data
        for (i, layer) in self.layers_data.iter().enumerate() {
            // Setup texture
            let texture_handle = asset_server.load(&layer.path);
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                layer.tile_size,
                layer.cols,
                layer.rows,
                None,
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let spritesheet_bundle = SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            };

            // Spawn a grid of textures, so that they convincingly wrap around the screen when scrolling.
            // For unidirectional layers, only spawn a single row or column in direction of their movement.

            // In every row of the grid, our goal is to have a central texture and at least two that surround it,
            // plus as much as it would take to fill the rest of the window space in both directions. Same logic
            // applies to vertical placement.

            let y_max_index = match layer.speed {
                layer::LayerSpeed::Vertical(_) | layer::LayerSpeed::Bidirectional(..) => max(
                    (window_size.y / (layer.tile_size.y * layer.scale) + 1.0) as i32,
                    1,
                ),
                layer::LayerSpeed::Horizontal(_) => 0,
            };
            let x_max_index = match layer.speed {
                layer::LayerSpeed::Horizontal(_) | layer::LayerSpeed::Bidirectional(..) => max(
                    (window_size.x / (layer.tile_size.x * layer.scale) + 1.0) as i32,
                    1,
                ),
                layer::LayerSpeed::Vertical(_) => 0,
            };
            let texture_count = Vec2::new(
                2.0 * x_max_index as f32 + 1.0,
                2.0 * y_max_index as f32 + 1.0,
            );

            // Spawn parallax layer entity
            let mut entity_commands = commands.spawn_empty();
            entity_commands
                .insert(Name::new(format!("Parallax Layer ({})", i)))
                .insert(RenderLayers::from_layers(&[render_layer]))
                .insert(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(layer.position.x, layer.position.y, layer.z),
                        scale: Vec3::new(layer.scale, layer.scale, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for j in -x_max_index..=x_max_index {
                        for k in -y_max_index..=y_max_index {
                            let mut adjusted_spritesheet_bundle = spritesheet_bundle.clone();
                            adjusted_spritesheet_bundle.transform.translation.x =
                                layer.tile_size.x * j as f32;
                            adjusted_spritesheet_bundle.transform.translation.y =
                                layer.tile_size.y * k as f32;
                            parent
                                .spawn(adjusted_spritesheet_bundle)
                                .insert(RenderLayers::from_layers(&[render_layer]))
                                .insert(layer::LayerTextureComponent {
                                    width: layer.tile_size.x,
                                    height: layer.tile_size.y,
                                });
                        }
                    }
                });

            // Add layer component to entity
            entity_commands.insert(layer::LayerComponent {
                speed: match layer.speed {
                    layer::LayerSpeed::Horizontal(vx) => Vec2::new(vx, 0.0),
                    layer::LayerSpeed::Vertical(vy) => Vec2::new(0.0, vy),
                    layer::LayerSpeed::Bidirectional(vx, vy) => Vec2::new(vx, vy),
                },
                texture_count,
                transition_factor: layer.transition_factor,
                camera: self.camera,
            });

            // Push parallax layer entity to layer_entities
            entities.push(entity_commands.id());
        }
        entities
    }
}

/// Event used to update parallax
pub struct ParallaxMoveEvent {
    /// Speed to move camera
    pub camera_move_speed: Vec2,

    pub camera: Entity,
}

/// Attach to a single camera to be used with parallax
#[derive(Component)]
pub struct ParallaxCameraComponent {
    pub render_layer: u8,
    pub entities: Vec<Entity>,
}

impl ParallaxCameraComponent {
    pub fn new(render_layer: u8) -> Self {
        Self {
            render_layer: render_layer,
            ..default()
        }
    }
}

impl Default for ParallaxCameraComponent {
    fn default() -> Self {
        Self {
            render_layer: 0,
            entities: vec![],
        }
    }
}
