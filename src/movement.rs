use bevy::prelude::*;
use crate::setup::{AnimationIndices, AnimationTimer, InactivityTimer, IsAttacking, Player};

pub(crate) fn movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut InactivityTimer,
        &mut IsAttacking
    ), With<Player>>,
) {
    for (mut transform, mut indices, mut timer, mut inactivity_timer, mut is_attacking) in &mut query {
        let mut direction = Vec3::ZERO;

        if is_attacking.0 {
            timer.unpause();
            if timer.finished() {
                is_attacking.0 = false;
            }
            return;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            indices.first = 36;
            indices.last = 41;
            timer.reset();
            is_attacking.0 = true;
            timer.unpause();
            return;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            transform.scale.x = -transform.scale.x.abs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            transform.scale.x = transform.scale.x.abs();
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        if direction != Vec3::ZERO {
            transform.translation += direction * time.delta_seconds() * 200.0;
            inactivity_timer.reset();

            if direction.x != 0.0 {
                if indices.first != 24 || indices.last != 29 {
                    indices.first = 24;
                    indices.last = 29;
                    timer.reset();
                }
            } else if direction.y > 0.0 {
                if indices.first != 30 || indices.last != 35 {
                    indices.first = 30;
                    indices.last = 35;
                    timer.reset();
                }
            } else if direction.y < 0.0 {
                if indices.first != 18 || indices.last != 23 {
                    indices.first = 18;
                    indices.last = 23;
                    timer.reset();
                }
            }

            timer.unpause();
        } else {
            inactivity_timer.tick(time.delta());
            if inactivity_timer.finished() {
                if indices.first != 0 || indices.last != 5 {
                    indices.first = 0;
                    indices.last = 5;
                    timer.reset();
                }
                timer.unpause();
            } else {
                timer.pause();
            }
        }
    }
}