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

#[derive(Debug, Deserialize, Clone)]
pub enum RepeatStrategy {
    Same,
    Mirror,
}

impl RepeatStrategy {
    pub fn transform(
        &self,
        mut spritesheet_bundle: SpriteSheetBundle,
        pos: (i32, i32),
    ) -> SpriteSheetBundle {
        match self {
            Self::Same => spritesheet_bundle,
            Self::Mirror => {
                let (x, y) = pos;
                spritesheet_bundle.sprite.flip_x = x % 2 != 0;
                spritesheet_bundle.sprite.flip_y = y % 2 != 0;
                spritesheet_bundle
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum LayerRepeat {
    Horizontal(RepeatStrategy),
    Vertical(RepeatStrategy),
    Bidirectional(RepeatStrategy, RepeatStrategy),
}

impl LayerRepeat {
    pub fn both(strategy: RepeatStrategy) -> Self {
        Self::Bidirectional(strategy.clone(), strategy)
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

    pub fn get_horizontal_strategy(&self) -> RepeatStrategy {
        match self {
            Self::Horizontal(strategy) => strategy.clone(),
            Self::Bidirectional(strategy, _) => strategy.clone(),
            _ => RepeatStrategy::Same,
        }
    }

    pub fn get_vertical_strategy(&self) -> RepeatStrategy {
        match self {
            Self::Vertical(strategy) => strategy.clone(),
            Self::Bidirectional(_, strategy) => strategy.clone(),
            _ => RepeatStrategy::Same,
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
}

impl Default for LayerData {
    fn default() -> Self {
        Self {
            speed: LayerSpeed::Horizontal(1.0),
            repeat: LayerRepeat::Bidirectional(RepeatStrategy::Same, RepeatStrategy::Same),
            path: "".to_string(),
            tile_size: Vec2::ZERO,
            cols: 1,
            rows: 1,
            scale: Vec2::ONE,
            z: 0.0,
            position: Vec2::ZERO,
            color: Color::WHITE,
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
