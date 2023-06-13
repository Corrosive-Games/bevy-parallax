use bevy::prelude::*;
use serde::Deserialize;

/// Layer speed type.
/// Layers with horizontal or vertical speed are only able to travel in one direction,
/// while bidirectional layers can be scrolled endlessly in both directions.
#[derive(Debug, Deserialize)]
pub enum LayerSpeed {
    Horizontal(f32),
    Vertical(f32),
    Bidirectional(f32, f32),
}

/// Layer initialization data
#[derive(Debug, Deserialize, Resource)]
#[serde(default)]
pub struct LayerData {
    /// Relative speed of layer to the camera movement.
    /// If the speed value is set to 1.0, the layer won't move in that direction.
    pub speed: LayerSpeed,
    /// Path to layer texture file
    pub path: String,
    /// Size of a tile of the texture
    pub tile_size: Vec2,
    /// Columns in the texture file
    pub cols: usize,
    /// Rows in the texture file
    pub rows: usize,
    /// Scale of the texture
    pub scale: f32,
    /// Z position of the layer
    pub z: f32,
    /// Default initial position of the Entity container
    pub position: Vec2,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,
}

impl Default for LayerData {
    fn default() -> Self {
        Self {
            speed: LayerSpeed::Horizontal(1.0),
            path: "".to_string(),
            tile_size: Vec2::ZERO,
            cols: 1,
            rows: 1,
            scale: 1.0,
            z: 0.0,
            position: Vec2::ZERO,
            transition_factor: 1.2,
        }
    }
}

/// Core component for parallax layer
#[derive(Component)]
pub struct LayerComponent {
    /// Relative speed of layer to the camera movement
    pub speed: Vec2,
    /// Number of rows (x) and columns (y) with the textures in the layer
    pub texture_count: Vec2,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,

    pub camera: Entity
}

/// Core component for layer texture
#[derive(Component)]
pub struct LayerTextureComponent {
    /// Width of the texture
    pub width: f32,

    /// Height of the texture
    pub height: f32,
}
