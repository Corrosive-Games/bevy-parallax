use bevy::{prelude::*};
use bevy::window::PrimaryWindow;

pub mod camera;
pub mod layer;
pub mod parallax;
pub mod sprite;

pub use camera::*;
pub use layer::*;
pub use parallax::*;
pub use sprite::*;

pub struct ParallaxPlugin;

impl ParallaxPlugin {
    #[cfg(feature = "bevy-inspector-egui")]
    fn add_features(&self, app: &mut App) {
        app.register_type::<Limit>()
            .register_type::<CameraFollow>()
            .register_type::<LayerComponent>()
            .register_type::<LayerTextureComponent>()
            .register_type::<ParallaxCameraComponent>();
    }

    #[cfg(not(feature = "bevy-inspector-egui"))]
    fn add_features(&self, _app: &mut App) {}
}

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ParallaxMoveEvent>()
            .add_event::<CreateParallaxEvent>()
            .add_systems(PreUpdate, create_parallax_system)
            .add_systems(Update, sprite_frame_update_system)
            .add_systems(
                Update,
                (camera_follow_system, move_layers_system, update_layer_textures_system)
                    .chain()
                    .in_set(ParallaxSystems),
            );
        self.add_features(app);
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ParallaxSystems;

fn create_parallax_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    parallax_query: Query<(Entity, &ParallaxCameraComponent, &Camera)>,
    layers_query: Query<(Entity, &LayerComponent)>,
    mut create_parallax_events: EventReader<CreateParallaxEvent>,
) {
    let mut window_size = Vec2::new(primary_window.width(), primary_window.height());
    for event in create_parallax_events.read() {
        if let Ok((parallax_entity, parallax, camera)) = parallax_query.get(event.camera) {
            for (entity, layer) in layers_query.iter() {
                // If it is not my layer don't despawn
                if layer.camera != parallax_entity {
                    continue;
                }
                commands.entity(entity).despawn_recursive();
            }
            if let Some(viewport) = &camera.viewport {
                window_size = viewport.physical_size.as_vec2();
            }
            event.create_layers(
                &mut commands,
                window_size,
                &asset_server,
                &mut texture_atlases,
                parallax.render_layer,
            );
        }
    }
}

/// Move camera and background layers
fn move_layers_system(
    mut camera_query: Query<(&mut Transform, &ParallaxCameraComponent)>,
    mut layer_query: Query<(&mut Transform, &LayerComponent), Without<ParallaxCameraComponent>>,
    mut move_events: EventReader<ParallaxMoveEvent>,
) {
    for event in move_events.read() {
        if let Ok((mut camera_transform, parallax)) = camera_query.get_mut(event.camera) {
            let camera_translation = camera_transform.translation.clone();
            camera_transform.translation = parallax
                .inside_limits(camera_transform.translation.truncate() + event.translation)
                .extend(camera_transform.translation.z);
            let real_translation = camera_transform.translation - camera_translation;
            camera_transform.rotate_z(event.rotation);
            for (mut layer_transform, layer) in layer_query.iter_mut() {
                if layer.camera != event.camera {
                    continue;
                }
                layer_transform.translation.x += real_translation.x * layer.speed.x;
                layer_transform.translation.y += real_translation.y * layer.speed.y;
            }
        }
    }
}

/// Update layer positions to keep the effect going indefinitely
fn update_layer_textures_system(
    layer_query: Query<(&LayerComponent, &Children)>,
    mut texture_query: Query<(&GlobalTransform, &mut Transform, &LayerTextureComponent, &ViewVisibility), Without<ParallaxCameraComponent>>,
    camera_query: Query<(Entity, &Transform, &Camera), With<ParallaxCameraComponent>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut move_events: EventReader<ParallaxMoveEvent>,
) {
    for event in move_events.read() {
        if !event.has_translation() {
            continue;
        }
        let primary_window = window_query.get_single().unwrap();
        let window_size = Vec2::new(primary_window.width(), primary_window.height());
        if let Ok((camera_entity, camera_transform, camera)) = camera_query.get(event.camera) {
            let view_size = match &camera.viewport {
                Some(viewport) => viewport.physical_size.as_vec2(),
                _ => window_size,
            };
            for (layer, children) in layer_query.iter() {
                if layer.camera != camera_entity {
                    continue;
                }
                for &child in children.iter() {
                    let (texture_gtransform, mut texture_transform, layer_texture, computed_visibility) =
                        texture_query.get_mut(child).unwrap();
                    // Do not move visible textures
                    if computed_visibility.get() {
                        continue;
                    }
                    let texture_gtransform = texture_gtransform.compute_transform();
                    let texture_translation = camera_transform.translation - texture_gtransform.translation;
                    if layer.repeat.has_horizontal() {
                        let x_delta = layer_texture.width * layer.texture_count.x;
                        let half_width = layer_texture.width * texture_gtransform.scale.x / 2.0;
                        // Move not visible right texture to left side of layer when camera is moving to left
                        if event.has_left_translation() && texture_translation.x + half_width < -view_size.x {
                            texture_transform.translation.x -= x_delta;
                        }
                        // Move not visible left texture to right side of layer when camera is moving to right
                        if event.has_right_translation() && texture_translation.x - half_width > view_size.x {
                            texture_transform.translation.x += x_delta;
                        }
                    }
                    if layer.repeat.has_vertical() {
                        let y_delta = layer_texture.height * layer.texture_count.y;
                        let half_height = layer_texture.height * texture_gtransform.scale.y / 2.0;
                        // Move not visible top texture to the bottom of the layer when the camera is moving to the bottom
                        if event.has_down_translation() && texture_translation.y + half_height < -view_size.y {
                            texture_transform.translation.y -= y_delta;
                        }
                        // Move not visible bottom texture to the top of the layer when the camera is moving to the top
                        if event.has_up_translation() && texture_translation.y - half_height > view_size.y {
                            texture_transform.translation.y += y_delta;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }
    external_doc_test!(include_str!("../README.md"));
}
