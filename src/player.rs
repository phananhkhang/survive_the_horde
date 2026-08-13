use crate::enemy::Enemy;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub hp: i32,
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    transform.translation +=
        direction.normalize_or_zero() * 300.0 * time.delta_secs();
    transform.translation.x = transform.translation.x.clamp(-590.0, 590.0);
    transform.translation.y = transform.translation.y.clamp(-310.0, 310.0);
}
