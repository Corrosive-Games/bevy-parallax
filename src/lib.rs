use bevy::{prelude::*, window::PrimaryWindow};

pub mod layer;
pub mod parallax;

pub use layer::*;
pub use parallax::*;

pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ParallaxMoveEvent>()
            .add_startup_system(initialize_parallax_system)
            .add_system(follow_camera_system.in_set(ParallaxSystems))
            .add_system(
                update_layer_textures_system
                    .in_set(ParallaxSystems)
                    .after(follow_camera_system),
            );
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ParallaxSystems;

/// Initialize the parallax resource
fn initialize_parallax_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut parallax_res: ResMut<ParallaxResource>,
) {
    let primary_window = window_query.get_single().unwrap();
    parallax_res.window_size = Vec2::new(primary_window.width(), primary_window.height());
    parallax_res.create_layers(&mut commands, &asset_server, &mut texture_atlases);
}

/// Move camera and background layers
fn follow_camera_system(
    mut camera_query: Query<&mut Transform, With<ParallaxCameraComponent>>,
    mut layer_query: Query<(&mut Transform, &LayerComponent), Without<ParallaxCameraComponent>>,
    mut move_events: EventReader<ParallaxMoveEvent>,
) {
    for event in move_events.iter() {
        if let Ok(mut camera_transform) = camera_query.get_mut(event.camera) {
            camera_transform.translation += event.camera_move_speed.extend(0.0);
            for (mut layer_transform, layer) in layer_query.iter_mut() {
                layer_transform.translation.x += event.camera_move_speed.x * layer.speed.x;
                layer_transform.translation.y += event.camera_move_speed.y * layer.speed.y;
            }
        }
    }
}

/// Update layer positions to keep the effect going indefinitely
fn update_layer_textures_system(
    layer_query: Query<(&LayerComponent, &Children)>,
    mut texture_query: Query<
        (&GlobalTransform, &mut Transform, &LayerTextureComponent),
        Without<ParallaxCameraComponent>,
    >,
    camera_query: Query<&Transform, With<ParallaxCameraComponent>>,
    parallax_resource: Res<ParallaxResource>,
) {
    if let Some(camera_transform) = camera_query.iter().next() {
        for (layer, children) in layer_query.iter() {
            for &child in children.iter() {
                let (texture_gtransform, mut texture_transform, layer_texture) =
                    texture_query.get_mut(child).unwrap();

                let texture_gtransform = texture_gtransform.compute_transform();

                // Move right-most texture to left side of layer when camera is approaching left-most end
                if camera_transform.translation.x - texture_gtransform.translation.x
                    + ((layer_texture.width * texture_gtransform.scale.x) / 2.0)
                    < -(parallax_resource.window_size.x * layer.transition_factor)
                {
                    texture_transform.translation.x -= layer_texture.width * layer.texture_count.x;
                // Move left-most texture to right side of layer when camera is approaching right-most end
                } else if camera_transform.translation.x
                    - texture_gtransform.translation.x
                    - ((layer_texture.width * texture_gtransform.scale.x) / 2.0)
                    > parallax_resource.window_size.x * layer.transition_factor
                {
                    texture_transform.translation.x += layer_texture.width * layer.texture_count.x;
                }

                // Move the top texture to the bottom of the layer when the camera is approaching the bottom
                if camera_transform.translation.y - texture_gtransform.translation.y
                    + ((layer_texture.height * texture_gtransform.scale.y) / 2.0)
                    < -(parallax_resource.window_size.y * layer.transition_factor)
                {
                    texture_transform.translation.y -= layer_texture.height * layer.texture_count.y;
                // Move the bottom texture to the top of the layer when the camera is approaching the top
                } else if camera_transform.translation.y
                    - texture_gtransform.translation.y
                    - ((layer_texture.height * texture_gtransform.scale.y) / 2.0)
                    > parallax_resource.window_size.y * layer.transition_factor
                {
                    texture_transform.translation.y += layer_texture.height * layer.texture_count.y;
                }
            }
        }
    }
}
