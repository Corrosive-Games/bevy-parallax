use std::time::Duration;

use bevy::prelude::*;
use serde::Deserialize;

use crate::SpriteFrameUpdate;

/// Layer speed type.
/// Layers with horizontal or vertical speed are only able to travel in one direction,
/// while bidirectional layers can be scrolled endlessly in both directions.
#[derive(Debug, Deserialize)]
pub enum LayerSpeed {
    Horizontal(f32),
    Vertical(f32),
    Bidirectional(f32, f32),
}

#[derive(Debug, Deserialize, Clone)]
pub enum RepeatStrategy {
    Same,
    MirrorHorizontally,
    MirrorVertically,
    MirrorBoth,
}

impl RepeatStrategy {
    pub fn transform(&self, sprite_sheet_bundle: &mut SpriteSheetBundle, pos: (i32, i32)) {
        match self {
            Self::Same => (),
            Self::MirrorHorizontally => {
                let (x, _) = pos;
                sprite_sheet_bundle.sprite.flip_x ^= x % 2 != 0;
            }
            Self::MirrorVertically => {
                let (_, y) = pos;
                sprite_sheet_bundle.sprite.flip_y ^= y % 2 != 0;
            }
            Self::MirrorBoth => {
                let (x, y) = pos;
                sprite_sheet_bundle.sprite.flip_x ^= x % 2 != 0;
                sprite_sheet_bundle.sprite.flip_y ^= y % 2 != 0;
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum LayerRepeat {
    Horizontal(RepeatStrategy),
    Vertical(RepeatStrategy),
    Bidirectional(RepeatStrategy),
}

impl LayerRepeat {
    pub fn both(strategy: RepeatStrategy) -> Self {
        Self::Bidirectional(strategy)
    }

    pub fn horizontally(strategy: RepeatStrategy) -> Self {
        Self::Horizontal(strategy)
    }

    pub fn vertically(strategy: RepeatStrategy) -> Self {
        Self::Vertical(strategy)
    }

    pub fn has_vertical(&self) -> bool {
        match self {
            Self::Horizontal(_) => false,
            _ => true,
        }
    }

    pub fn has_horizontal(&self) -> bool {
        match self {
            Self::Vertical(_) => false,
            _ => true,
        }
    }

    pub fn get_strategy(&self) -> RepeatStrategy {
        match self {
            Self::Horizontal(strategy) => strategy.clone(),
            Self::Bidirectional(strategy) => strategy.clone(),
            Self::Vertical(strategy) => strategy.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Resource)]
pub enum Animation {
    FPS(f32),
    FrameDuration(Duration),
    TotalDuration(Duration),
}

impl Animation {
    pub fn to_sprite_update(&self, layer_data: &LayerData) -> SpriteFrameUpdate {
        let total = layer_data.cols * layer_data.rows;
        let duration = match self {
            Self::FPS(fps) => Duration::from_secs_f32(1. / fps),
            Self::FrameDuration(duration) => duration.clone(),
            Self::TotalDuration(duration) => duration.div_f32(total as f32),
        };
        SpriteFrameUpdate {
            total,
            index: layer_data.index,
            timer: Timer::new(duration, TimerMode::Repeating),
        }
    }
}

/// Layer initialization data
#[derive(Debug, Deserialize, Resource)]
#[serde(default)]
pub struct LayerData {
    /// Relative speed of layer to the camera movement.
    /// If the speed value is set to 1.0, the layer won't move in that direction.
    pub speed: LayerSpeed,

    pub repeat: LayerRepeat,

    /// Path to layer texture file
    pub path: String,
    /// Size of a tile of the texture
    pub tile_size: Vec2,
    /// Columns in the texture file
    pub cols: usize,
    /// Rows in the texture file
    pub rows: usize,
    /// Scale of the texture
    pub scale: Vec2,
    /// Z position of the layer
    pub z: f32,
    /// Default initial position of the Entity container
    pub position: Vec2,

    pub color: Color,

    pub index: usize,

    pub flip: (bool, bool),

    pub animation: Option<Animation>,
}

impl LayerData {
    pub fn create_texture_atlas_layout(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            self.tile_size,
            self.cols,
            self.rows,
            None,
            None,
        )
    }

    pub fn crate_layer_texture(&self) -> LayerTextureComponent {
        LayerTextureComponent {
            width: self.tile_size.x,
            height: self.tile_size.y,
        }
    }

    pub fn create_animation_bundle(&self) -> Option<impl Bundle> {
        self.animation
            .as_ref()
            .map(|animation| animation.to_sprite_update(self))
    }
}

impl Default for LayerData {
    fn default() -> Self {
        Self {
            speed: LayerSpeed::Horizontal(1.0),
            repeat: LayerRepeat::Bidirectional(RepeatStrategy::Same),
            path: "".to_string(),
            tile_size: Vec2::ZERO,
            cols: 1,
            rows: 1,
            scale: Vec2::ONE,
            z: 0.0,
            position: Vec2::ZERO,
            color: Color::WHITE,
            index: 0,
            flip: (false, false),
            animation: None,
        }
    }
}

/// Core component for parallax layer
#[derive(Component)]
pub struct LayerComponent {
    /// Relative speed of layer to the camera movement
    pub speed: Vec2,
    ///
    pub repeat: LayerRepeat,
    /// Number of rows (x) and columns (y) with the textures in the layer
    pub texture_count: Vec2,

    pub camera: Entity,
}

/// Core component for layer texture
#[derive(Component)]
pub struct LayerTextureComponent {
    /// Width of the texture
    pub width: f32,

    /// Height of the texture
    pub height: f32,
}
