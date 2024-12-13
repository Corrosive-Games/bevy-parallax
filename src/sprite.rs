use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteFrameUpdate {
    pub index: usize,
    pub total: usize,
    pub timer: Timer,
}

impl SpriteFrameUpdate {
    pub fn next_index(&mut self, duration: Duration) -> usize {
        self.timer.tick(duration);
        if self.timer.just_finished() {
            self.index += 1;
        }
        self.index % self.total
    }
}

pub fn sprite_frame_update_system(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteFrameUpdate)>,
) {
    for (mut sprite, mut frame) in query.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = frame.next_index(time.delta());
        }

    }
}
