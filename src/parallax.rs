use std::cmp::max;

use crate::layer;
use bevy::prelude::*;

/// Event used to update parallax
pub struct ParallaxMoveEvent {
    /// Speed to move camera
    pub camera_move_speed: Vec2,
}

/// Resource for managing parallax
#[derive(Debug, Resource)]
pub struct ParallaxResource {
    /// Data to describe each layer of parallax
    pub layer_data: Vec<layer::LayerData>,
    /// Parallax layer entities
    pub layer_entities: Vec<Entity>,
    /// Dimensions of window
    pub window_size: Vec2,
}

impl Default for ParallaxResource {
    fn default() -> Self {
        Self {
            layer_data: vec![],
            layer_entities: vec![],
            window_size: Vec2::ZERO,
        }
    }
}

impl ParallaxResource {
    /// Create a new parallax resource
    pub fn new(layer_data: Vec<layer::LayerData>) -> Self {
        ParallaxResource {
            layer_data,
            layer_entities: vec![],
            window_size: Vec2::ZERO,
        }
    }

    /// Delete all layer entities in parallax resource and empty Vec
    pub fn despawn_layers(&mut self, commands: &mut Commands) {
        // Remove all layer entities
        for entity in self.layer_entities.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        // Empty the layer entity vector
        self.layer_entities = vec![];
    }

    /// Create layers from layer data
    pub fn create_layers(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) {
        // Despawn any existing layers
        self.despawn_layers(commands);

        // Spawn new layers using layer_data
        for (i, layer) in self.layer_data.iter().enumerate() {
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
                sprite: TextureAtlasSprite {
                    color: layer.color,
                    ..default()
                },
                ..Default::default()
            };

            // Spawn a grid of textures, so that they convincingly wrap around the screen when scrolling.
            // For unidirectional layers, only spawn a single row or column in direction of their movement.

            // In every row of the grid, our goal is to have a central texture and at least two that surround it,
            // plus as much as it would take to fill the rest of the window space in both directions. Same logic
            // applies to vertical placement.

            let y_max_index = match layer.speed {
                layer::LayerSpeed::Vertical(_) | layer::LayerSpeed::Bidirectional(..) => max(
                    (self.window_size.y / (layer.tile_size.y * layer.scale) + 1.0) as i32,
                    1,
                ),
                layer::LayerSpeed::Horizontal(_) => 0,
            };
            let x_max_index = match layer.speed {
                layer::LayerSpeed::Horizontal(_) | layer::LayerSpeed::Bidirectional(..) => max(
                    (self.window_size.x / (layer.tile_size.x * layer.scale) + 1.0) as i32,
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
                            parent.spawn(adjusted_spritesheet_bundle).insert(
                                layer::LayerTextureComponent {
                                    width: layer.tile_size.x,
                                    height: layer.tile_size.y,
                                },
                            );
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
            });

            // Push parallax layer entity to layer_entities
            self.layer_entities.push(entity_commands.id());
        }
    }
}

/// Attach to a single camera to be used with parallax
#[derive(Component)]
pub struct ParallaxCameraComponent;
