use std::f32::consts::PI;

use bevy::prelude::*;

use crate::ParallaxMoveEvent;

pub enum RotationStrategy {
    None,
    Fixed,
    P(f32),
    PID(f32, f32, f32, f32, f32),
}

fn shortest_angle(a: f32, b: f32) -> f32 {
    let mut diff = a - b;
    if diff > PI {
        diff -= PI * 2.;
    }
    if diff < -PI {
        diff += PI * 2.;
    }
    diff
}

impl RotationStrategy {
    pub fn rotation(&mut self, delta_time: f32, target: f32, current: f32) -> f32 {
        match self {
            Self::None => 0.,
            Self::Fixed => target - current,
            Self::P(kp) => (target - current) * kp.clone() * delta_time,
            Self::PID(kp, ki, kd, last_error, acc_error) => {
                let error = shortest_angle(target, current);
                let p_value = error * delta_time * kp.clone();
                let d_value = if delta_time != 0. {
                    (error - last_error.clone()) / delta_time * kd.clone()
                } else {
                    0.
                };
                *acc_error += error;
                let i_value = acc_error.clone() * delta_time * ki.clone();
                *last_error = error;
                println!("{p_value} + {i_value} + {d_value}");
                p_value + i_value + d_value
            }
        }
    }
}

pub enum TranslateStrategy {
    None,
    Fixed,
    P(f32),
    PID(f32, f32, f32, Vec2, Vec2),
}

impl TranslateStrategy {
    pub fn translation(&mut self, delta_time: f32, target: Vec2, current: Vec2) -> Vec2 {
        match self {
            Self::None => Vec2::ZERO,
            Self::Fixed => target - current,
            Self::P(kp) => (target - current) * kp.clone() * delta_time,
            Self::PID(kp, ki, kd, last_error, acc_error) => {
                println!("{delta_time}");
                let error = target - current;
                let p_value = error * delta_time * kp.clone();
                let d_value = if delta_time != 0. {
                    (error - last_error.clone()) / delta_time * kd.clone()
                } else {
                    Vec2::ZERO
                };
                *acc_error += error;
                let i_value = acc_error.clone() * delta_time * ki.clone();
                *last_error = error;
                println!("{p_value} + {i_value} + {d_value}");
                p_value + i_value + d_value
            }
        }
    }
}

#[derive(Component)]
pub struct CameraFollow {
    pub target: Entity,
    pub translate_strategy: TranslateStrategy,
    pub rotation_strategy: RotationStrategy,
}

impl CameraFollow {
    pub fn fixed(entity: Entity) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::Fixed,
            rotation_strategy: RotationStrategy::Fixed,
        }
    }

    pub fn proportional(entity: Entity, value: f32) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::P(value),
            rotation_strategy: RotationStrategy::P(value),
        }
    }

    pub fn pid(entity: Entity, pid: Vec3) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::PID(pid.x, pid.y, pid.z, Vec2::ZERO, Vec2::ZERO),
            rotation_strategy: RotationStrategy::PID(pid.x, pid.y, pid.z, 0., 0.),
        }
    }

    pub fn fixed_translation(entity: Entity) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::Fixed,
            rotation_strategy: RotationStrategy::None,
        }
    }

    pub fn proportional_translation(entity: Entity, value: f32) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::P(value),
            rotation_strategy: RotationStrategy::None,
        }
    }

    pub fn pid_translation(entity: Entity, pid: Vec3) -> Self {
        Self {
            target: entity,
            translate_strategy: TranslateStrategy::PID(pid.x, pid.y, pid.z, Vec2::ZERO, Vec2::ZERO),
            rotation_strategy: RotationStrategy::None,
        }
    }
}

pub fn camera_follow_system(
    transform_query: Query<&Transform>,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut CameraFollow)>,
    mut event_writer: EventWriter<ParallaxMoveEvent>,
) {
    for (camera, camera_transform, mut follow) in query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(follow.target) {
            let seconds = time.delta_seconds();
            let camera_movement = follow.translate_strategy.translation(
                seconds,
                target_transform.translation.truncate(),
                camera_transform.translation.truncate(),
            );
            let camera_rotation = follow.rotation_strategy.rotation(
                seconds,
                target_transform.rotation.to_euler(EulerRot::XYZ).2,
                camera_transform.rotation.to_euler(EulerRot::XYZ).2,
            );
            event_writer.send(ParallaxMoveEvent {
                translation: camera_movement,
                camera: camera,
                rotation: camera_rotation,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::shortest_angle;

    #[test]
    fn test_shortest_angle() {
        assert_eq!(shortest_angle(0., 0.), 0.);
        assert_eq!(shortest_angle(PI, 0.), PI);
        assert_eq!(shortest_angle(-PI, 0.), -PI);
        assert_eq!(shortest_angle(PI * 2., 0.), 0.);
        assert_eq!(shortest_angle(PI * -2., 0.), 0.);

        assert_eq!(
            shortest_angle(f32::to_radians(10.), f32::to_radians(0.)),
            f32::to_radians(10.)
        );
        assert_eq!(
            shortest_angle(-f32::to_radians(10.), f32::to_radians(0.)),
            -f32::to_radians(10.)
        );
    }
}
