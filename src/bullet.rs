use crate::enemy::Enemy;
use crate::player::Player;
use bevy::prelude::*;
use rand::seq::IndexedRandom;
#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
}
pub fn bullet_movement(
    mut bullets: Query<(Entity, &mut Transform, &Bullet), Without<Enemy>>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy), Without<Bullet>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Với từng viên đạn đang tồn tại
    for (bullet_entity, mut bullet_transform, bullet) in &mut bullets {
        // Cho viên đạn di chuyển
        bullet_transform.translation += bullet.direction.extend(0.0) * 600.0 * time.delta_secs();
        // Kiểm tra va chạm với từng kẻ địch
        for (enemy_entity, enemy_transform, mut enemy) in &mut enemies {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);
            if distance <= 30.0 {
                enemy.hp -= 1;
                commands.entity(bullet_entity).despawn();
                if enemy.hp <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
                break; // Nếu viên đạn đã va chạm với một kẻ địch, không cần kiểm tra với các kẻ địch khác
            }
        }
    }
}
pub fn shoot(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemies_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    if !mouse.just_pressed(MouseButton::Left) && !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let enemy_position: Vec<Vec3> = enemies_query
        .iter()
        .map(|transform| transform.translation)
        .collect();
    if enemy_position.is_empty() {
        return;
    }
    let mut rng = rand::rng();
    if let Some(&target_position) = enemy_position.choose(&mut rng) {
        let direction = (target_position - player_transform.translation)
            .truncate()
            .normalize_or_zero();
        commands.spawn((
            Bullet { direction },
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(10.0, 10.0)),
            Transform::from_translation(player_transform.translation),
        ));
    }
}
