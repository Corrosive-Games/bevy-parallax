use std::f32::consts::PI;

use bevy::prelude::*;
#[cfg(feature = "bevy-inspector-egui")]
use bevy_inspector_egui::prelude::*;

use crate::{Limit, ParallaxMoveEvent};

#[derive(Clone, Copy)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    clegg_integrator: bool,
    integral_limit: Option<Limit>,
}

impl Default for PID {
    fn default() -> Self {
        Self {
            kp: 0.1,
            ki: 0.1,
            kd: 0.001,
            clegg_integrator: false,
            integral_limit: Option::None,
        }
    }
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            ..default()
        }
    }

    pub fn with_integral_limit(mut self, limit: Limit) -> Self {
        self.integral_limit = Some(limit);
        self
    }

    pub fn create_radial(&self) -> RotationStrategy {
        RotationStrategy::PID {
            kp: self.kp,
            ki: self.ki,
            kd: self.kd,
            last_error: 0.,
            integral: 0.,
            clegg_integrator: self.clegg_integrator,
            integral_limit: match &self.integral_limit {
                Some(limit) => limit.clone(),
                None => Limit::new(-PI, PI),
            },
        }
    }

    pub fn create_linear(&self) -> LinearAxisStrategy {
        LinearAxisStrategy::PID {
            kp: self.kp,
            ki: self.ki,
            kd: self.kd,
            last_error: 0.,
            integral: 0.,
            clegg_integrator: self.clegg_integrator,
            integral_limit: match &self.integral_limit {
                Some(limit) => limit.clone(),
                None => Limit::default(),
            },
        }
    }
}

#[derive(Default)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub enum RotationStrategy {
    #[default]
    None,
    Fixed,
    P(f32),
    PID {
        kp: f32,
        ki: f32,
        kd: f32,
        last_error: f32,
        integral: f32,
        clegg_integrator: bool,
        integral_limit: Limit,
    },
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
            Self::Fixed => shortest_angle(target, current),
            Self::P(kp) => shortest_angle(target, current) * kp.clone(),
            Self::PID {
                kp,
                ki,
                kd,
                last_error,
                integral,
                clegg_integrator,
                integral_limit,
            } => {
                let error = shortest_angle(target, current);
                let p_value = error * kp.clone();
                let d_value = if delta_time != 0. {
                    (error - last_error.clone()) / delta_time * kd.clone()
                } else {
                    0.
                };
                if *clegg_integrator && error.signum() != last_error.signum() {
                    *integral = 0.;
                } else {
                    *integral = integral_limit.fix(*integral + error * delta_time);
                }
                let i_value = integral.clone() * ki.clone();
                *last_error = error;
                p_value + i_value + d_value
            }
        }
    }
}

#[derive(Default, Clone)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub enum LinearAxisStrategy {
    None,
    #[default]
    Fixed,
    P(f32),
    PID {
        kp: f32,
        ki: f32,
        kd: f32,
        last_error: f32,
        integral: f32,
        clegg_integrator: bool,
        integral_limit: Limit,
    },
}

impl LinearAxisStrategy {
    pub fn compute(&mut self, delta_time: f32, target: f32, current: f32) -> f32 {
        match self {
            Self::None => 0.,
            Self::Fixed => target - current,
            Self::P(kp) => (target - current) * kp.clone(),
            Self::PID {
                kp,
                ki,
                kd,
                last_error,
                integral,
                clegg_integrator,
                integral_limit,
            } => {
                let error = target - current;
                let p_value = error * kp.clone();
                let d_value = if delta_time != 0. {
                    (error - last_error.clone()) / delta_time * kd.clone()
                } else {
                    0.
                };
                if *clegg_integrator && error.signum() != last_error.signum() {
                    *integral = 0.
                } else {
                    *integral = integral_limit.fix(*integral + error * delta_time);
                }
                let i_value = integral.clone() * ki.clone();
                *last_error = error;
                p_value + i_value + d_value
            }
        }
    }
}

#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct TranslationStrategy {
    pub x: LinearAxisStrategy,
    pub y: LinearAxisStrategy,
}

impl TranslationStrategy {
    pub fn new(x: LinearAxisStrategy, y: LinearAxisStrategy) -> Self {
        Self { x, y }
    }

    pub fn translation(&mut self, seconds: f32, target: Vec2, current: Vec2) -> Vec2 {
        Vec2::new(
            self.x.compute(seconds, target.x, current.x),
            self.y.compute(seconds, target.y, current.y),
        )
    }
}

#[derive(Component)]
#[cfg_attr(feature = "bevy-inspector-egui", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "bevy-inspector-egui", reflect(InspectorOptions))]
pub struct CameraFollow {
    pub target: Entity,
    pub translation_strategy: TranslationStrategy,
    pub rotation_strategy: RotationStrategy,
    pub offset: Vec2,
}

impl Default for CameraFollow {
    fn default() -> Self {
        Self {
            target: Entity::from_raw(0),
            translation_strategy: TranslationStrategy::new(LinearAxisStrategy::Fixed, LinearAxisStrategy::Fixed),
            rotation_strategy: RotationStrategy::None,
            offset: Vec2::ZERO,
        }
    }
}

impl CameraFollow {
    pub fn new(entity: Entity) -> Self {
        Self {
            target: entity,
            ..default()
        }
    }

    pub fn with_rotation(mut self, rotation: RotationStrategy) -> Self {
        self.rotation_strategy = rotation;
        self
    }

    pub fn with_translation(mut self, translate: TranslationStrategy) -> Self {
        self.translation_strategy = translate;
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn fixed(entity: Entity) -> Self {
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(LinearAxisStrategy::Fixed, LinearAxisStrategy::Fixed),
            rotation_strategy: RotationStrategy::Fixed,
            ..default()
        }
    }

    pub fn proportional(entity: Entity, value: f32) -> Self {
        let axis_strategy = LinearAxisStrategy::P(value);
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(axis_strategy.clone(), axis_strategy),
            rotation_strategy: RotationStrategy::P(value),
            ..default()
        }
    }

    pub fn pid(entity: Entity, pid: &PID) -> Self {
        let axis_strategy = pid.create_linear();
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(axis_strategy.clone(), axis_strategy),
            rotation_strategy: pid.create_radial(),
            ..default()
        }
    }

    pub fn pid_xyz(entity: Entity, x: &PID, y: &PID, z: &PID) -> Self {
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(x.create_linear(), y.create_linear()),
            rotation_strategy: z.create_radial(),
            ..default()
        }
    }

    pub fn fixed_translation(entity: Entity) -> Self {
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(LinearAxisStrategy::Fixed, LinearAxisStrategy::Fixed),
            rotation_strategy: RotationStrategy::None,
            ..default()
        }
    }

    pub fn proportional_translation(entity: Entity, value: f32) -> Self {
        let axis_strategy = LinearAxisStrategy::P(value);
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(axis_strategy.clone(), axis_strategy),
            rotation_strategy: RotationStrategy::None,
            ..default()
        }
    }

    pub fn pid_translation(entity: Entity, pid: PID) -> Self {
        let axis_strategy = pid.create_linear();
        Self {
            target: entity,
            translation_strategy: TranslationStrategy::new(axis_strategy.clone(), axis_strategy),
            rotation_strategy: RotationStrategy::None,
            ..default()
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
            let seconds = time.delta_secs();
            let target = target_transform.mul_transform(Transform::from_translation(follow.offset.extend(0.)));
            let camera_movement =
                follow
                    .translation_strategy
                    .translation(seconds, target.translation.truncate(), camera_transform.translation.truncate());
            let camera_rotation = follow.rotation_strategy.rotation(
                seconds,
                target.rotation.to_euler(EulerRot::XYZ).2,
                camera_transform.rotation.to_euler(EulerRot::XYZ).2,
            );
            event_writer.send(ParallaxMoveEvent {
                translation: camera_movement,
                camera,
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

        assert_eq!(shortest_angle(f32::to_radians(10.), f32::to_radians(0.)), f32::to_radians(10.));
        assert_eq!(shortest_angle(-f32::to_radians(10.), f32::to_radians(0.)), -f32::to_radians(10.));
    }
}
