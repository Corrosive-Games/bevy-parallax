use bevy::prelude::*;

/// Resource for managing parallax
pub struct ParallaxResource {
    pub layer_data: Vec<LayerData>,
    layer_entities: Vec<Entity>,
}

impl ParallaxResource {
    /// Create a new parallax resource
    pub fn new(layer_data: Vec<LayerData>) -> Self {
        ParallaxResource {
            layer_data,
            layer_entities: vec![],
        }
    }

    /// Delete all layer entities in parallax resource and empty Vec
    pub fn despawn_layers(&mut self, commands: &mut Commands) {
        for entity in self.layer_entities.iter() {
            commands.entity(*entity).despawn();
        }

        self.layer_entities = vec![];
    }

    /// Create layers from layer data
    pub fn create_layers(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) {
        // despawn and existing layers
        self.despawn_layers(commands);

        // spawn new layers using layer_data
        for (i, layer) in self.layer_data.iter().enumerate() {
            let texture_handle = asset_server.load(&layer.path);
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, layer.tile_size, layer.cols, layer.rows);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            commands
                .spawn()
                .insert(LayerComponent { speed: layer.speed })
                .insert(Name::new(format!("Parallax ({})", i)))
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, layer.z),
                        scale: Vec3::new(layer.scale, layer.scale, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }
}

/// Layer initialization data
pub struct LayerData {
    pub speed: f32,
    pub path: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub scale: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct LayerComponent {
    speed: f32,
}

pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_parallax_system);
    }
}

fn initialize_parallax_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut parallax_res: ResMut<ParallaxResource>,
) {
    parallax_res.create_layers(&mut commands, &asset_server, &mut texture_atlases);
}
