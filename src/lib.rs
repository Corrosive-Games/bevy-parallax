use bevy::prelude::*;

pub mod layer;
pub mod parallax;

pub use layer::*;
pub use parallax::*;

pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<parallax::ParallaxMoveEvent>()
            .add_startup_system(initialize_parallax_system)
            .add_system(follow_camera_system.label("follow_camera"))
            .add_system(update_layer_textures_system.after("follow_camera"));
    }
}

/// Initialize the parallax resource
fn initialize_parallax_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
    mut parallax_res: ResMut<parallax::ParallaxResource>,
) {
    let window = windows.get_primary().unwrap();
    parallax_res.window_size = Vec2::new(window.width(), window.height());
    parallax_res.create_layers(&mut commands, &asset_server, &mut texture_atlases);
}

/// Move camera and background layers
fn follow_camera_system(
    mut camera_query: Query<&mut Transform, With<parallax::ParallaxCameraComponent>>,
    mut layer_query: Query<
        (&mut Transform, &layer::LayerComponent),
        Without<parallax::ParallaxCameraComponent>,
    >,
    mut move_events: EventReader<parallax::ParallaxMoveEvent>,
) {
    if let Some(mut camera_transform) = camera_query.iter_mut().next() {
        for event in move_events.iter() {
            camera_transform.translation.x += event.camera_move_speed;
            for (mut layer_transform, layer) in layer_query.iter_mut() {
                layer_transform.translation.x += event.camera_move_speed * layer.speed;
            }
        }
    }
}

/// Update layer positions to keep the effect going indefinitely
fn update_layer_textures_system(
    layer_query: Query<(&layer::LayerComponent, &Children)>,
    mut texture_query: Query<
        (
            &GlobalTransform,
            &mut Transform,
            &layer::LayerTextureComponent,
        ),
        Without<parallax::ParallaxCameraComponent>,
    >,
    camera_query: Query<&Transform, With<parallax::ParallaxCameraComponent>>,
    parallax_resource: Res<parallax::ParallaxResource>,
) {
    if let Some(camera_transform) = camera_query.iter().next() {
        for (layer, children) in layer_query.iter() {
            for &child in children.iter() {
                let (texture_gtransform, mut texture_transform, layer_texture) =
                    texture_query.get_mut(child).unwrap();

                // Move right-most texture to left side of layer when camera is approaching left-most end
                if camera_transform.translation.x - texture_gtransform.translation.x
                    + ((layer_texture.width * texture_gtransform.scale.x) / 2.0)
                    < -(parallax_resource.window_size.x * layer.transition_factor)
                {
                    texture_transform.translation.x -= layer_texture.width * layer.texture_count;
                // Move left-most texture to right side of layer when camera is approaching right-most end
                } else if camera_transform.translation.x
                    - texture_gtransform.translation.x
                    - ((layer_texture.width * texture_gtransform.scale.x) / 2.0)
                    > parallax_resource.window_size.x * layer.transition_factor
                {
                    texture_transform.translation.x += layer_texture.width * layer.texture_count;
                }
            }
        }
    }
}
