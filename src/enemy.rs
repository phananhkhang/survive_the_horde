use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub hp: i32,
}

pub fn enemy_movement(
    mut enemies: Query<
        (&mut Transform, &Enemy, Entity),
        (With<Enemy>, Without<Player>),
    >,
    mut player: Query<
        (Entity, &Transform, &mut Player),
        (With<Player>, Without<Enemy>),
    >,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let Ok((player_entity, player_transform, mut player)) = player.single_mut() else {
        return;
    };

    for (mut enemy_transform, _enemy, enemy_entity) in &mut enemies {
        let direction =
            player_transform.translation - enemy_transform.translation;
        let direction = direction.normalize_or_zero();

        enemy_transform.translation +=
            direction * 50.0 * time.delta_secs();

        if enemy_transform
            .translation
            .distance(player_transform.translation)
            <= 40.0
        {
            commands.entity(enemy_entity).despawn();
            player.hp -= 1;

            if player.hp <= 0 {
                player.hp = 0;
                commands.entity(player_entity).despawn();
                next_state.set(GameState::GameOver);
            }
        }
    }
}
