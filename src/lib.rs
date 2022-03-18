use bevy::{prelude::*, render::view::Layer};

/// Resource for managing parallax
#[derive(Debug, Default)]
pub struct ParallaxResource {
    pub layer_data: Vec<LayerData>,
    pub layer_entities: Vec<Entity>,
    pub window_size: Vec2,
    pub camera_name: Option<String>,
}

impl ParallaxResource {
    /// Create a new parallax resource
    pub fn new(layer_data: Vec<LayerData>) -> Self {
        ParallaxResource {
            layer_data,
            layer_entities: vec![],
            window_size: Vec2::ZERO,
            camera_name: None,
        }
    }

    /// Delete all layer entities in parallax resource and empty Vec
    pub fn despawn_layers(&mut self, commands: &mut Commands) {
        for entity in self.layer_entities.iter() {
            commands.entity(*entity).despawn_recursive();
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
            let spritesheet_bundle = SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            };
            let mut entity_commands = commands.spawn();
            entity_commands
                .insert(LayerComponent { speed: layer.speed })
                .insert(Name::new(format!("Parallax Layer ({})", i)))
                .insert(Transform {
                    translation: Vec3::new(0.0, 0.0, layer.z),
                    scale: Vec3::new(layer.scale, layer.scale, 1.0),
                    ..Default::default()
                })
                .insert(GlobalTransform::default())
                .with_children(|parent| {
                    parent
                        .spawn_bundle(spritesheet_bundle.clone())
                        .insert(LayerTextureComponent);

                    let mut max_x = (layer.tile_size.x / 2.0) * layer.scale;

                    let mut adjusted_spritesheet_bundle = spritesheet_bundle.clone();

                    // spawn 2 windows length of background textures
                    while max_x < self.window_size.x {
                        adjusted_spritesheet_bundle.transform.translation.x += layer.tile_size.x;
                        max_x += layer.tile_size.x * layer.scale;
                        parent
                            .spawn_bundle(adjusted_spritesheet_bundle.clone())
                            .insert(LayerTextureComponent);

                        parent
                            .spawn_bundle({
                                let mut bundle = adjusted_spritesheet_bundle.clone();
                                bundle.transform.translation.x *= -1.0;
                                bundle
                            })
                            .insert(LayerTextureComponent);
                    }
                });
            self.layer_entities.push(entity_commands.id());
        }
    }
}

/// Layer initialization data
#[derive(Debug)]
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
    speed: f32, // speed of layer relative to camera
}

#[derive(Component)]
pub struct LayerTextureComponent;

pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ParallaxMoveEvent>()
            .add_startup_system(initialize_parallax_system)
            .add_system(follow_camera_system);
    }
}

fn initialize_parallax_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
    mut parallax_res: ResMut<ParallaxResource>,
) {
    let window = windows.get_primary().unwrap();
    parallax_res.window_size = Vec2::new(window.width(), window.height());
    parallax_res.create_layers(&mut commands, &asset_server, &mut texture_atlases);

    // TODO: Remove
    println!("{:?}", parallax_res);
}

pub struct ParallaxMoveEvent {
    pub camera_move_speed: f32,
}

fn follow_camera_system(
    mut camera_query: Query<(&mut Transform, &Camera)>,
    mut layer_query: Query<(&mut Transform, &LayerComponent), Without<Camera>>,
    parallax_resource: Res<ParallaxResource>,
    mut move_events: EventReader<ParallaxMoveEvent>,
) {
    for (mut camera_transform, camera) in camera_query.iter_mut() {
        if (parallax_resource.camera_name.is_some()
            && parallax_resource.camera_name.as_ref().unwrap() == camera.name.as_ref().unwrap())
            || (parallax_resource.camera_name.is_none()
                && camera.name.as_ref().unwrap() == "camera_2d")
        {
            for event in move_events.iter() {
                camera_transform.translation.x += event.camera_move_speed;
                for (mut layer_transform, layer) in layer_query.iter_mut() {
                    layer_transform.translation.x += event.camera_move_speed * layer.speed;
                }
            }

            break;
        } else {
            println!("camera not found")
        }
    }
}

/*
fn update_layer_textures(
    layer_query: Query<(&Transform, &Children), With<LayerComponent>>,
    mut texture_query: Query<&Transform, With<LayerTextureComponent>>,
) {
    for (layer_transform, children) in layer_query.iter() {
        for &child in children.iter() {
            let texture_transform = texture_query.get(child);
        }
    }
}
*/
