use bevy::prelude::*;
use serde::Deserialize;

/// Layer initialization data
#[derive(Debug, Deserialize)]
pub struct LayerData {
    pub speed: f32,
    pub path: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub scale: f32,
    pub z: f32,
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
            transition_factor: 1.2,
        }
    }
}

#[derive(Component)]
pub struct LayerComponent {
    pub speed: f32, // speed of layer relative to camera
    pub texture_count: f32,
    pub transition_factor: f32,
}

#[derive(Component)]
pub struct LayerTextureComponent {
    pub width: f32,
}
