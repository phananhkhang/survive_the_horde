use bevy::prelude::*;
mod bullet;
mod enemy;
mod player;
use bullet::{Bullet, bullet_movement, shoot};
use enemy::{Enemy, enemy_movement};
use player::{Player, player_movement};
use rand::RngExt;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_movement, enemy_movement, bullet_movement, shoot),
        )
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player { hp: 10 },
        Sprite::from_color(Color::srgb(0.0, 0.0, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let mut rng = rand::rng();
    for _ in 0..20 {
        let x = rng.random_range(-600.0..600.0);
        let y = rng.random_range(-400.0..400.0);
        commands.spawn((
            Enemy { hp: 2 },
            Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(30.0, 30.0)),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}
