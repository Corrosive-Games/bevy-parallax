use bevy::prelude::*;
use serde::Deserialize;

/// Layer initialization data
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LayerData {
    /// Relative speed of layer to the camera movement
    pub speed: f32,
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
            speed: 1.0,
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
    pub speed: f32,
    /// Number of textures in the layer
    pub texture_count: f32,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,
}

/// Core component for layer texture
#[derive(Component)]
pub struct LayerTextureComponent {
    /// Width of the texture
    pub width: f32,
}
