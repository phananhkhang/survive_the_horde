use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod bullet;
mod enemy;
mod player;
mod ui;

use bullet::{bullet_movement, shoot};
use enemy::{enemy_movement, Enemy};
use player::{player_movement, Player};
use ui::game_over::setup_game_over;
use ui::hud::{setup_hud, update_hud};
use ui::menu::{button_system, setup_menu};
use ui::win::setup_win;

use rand::RngExt;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Win,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct GameBackground;

#[derive(Component)]
pub struct UiRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<Score>()
        // Camera exists for the whole app.
        .add_systems(Startup, setup_camera)
        // Each screen gets built only when entering its state.
        .add_systems(
            OnEnter(GameState::Menu),
            (cleanup_game, setup_menu).chain(),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_ui)
        .add_systems(
            OnEnter(GameState::Playing),
            (cleanup_game, setup_game, setup_hud).chain(),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_ui)
        .add_systems(
            OnEnter(GameState::Win),
            (cleanup_game, setup_win).chain(),
        )
        .add_systems(OnExit(GameState::Win), cleanup_ui)
        .add_systems(
            OnEnter(GameState::GameOver),
            (cleanup_game, setup_game_over).chain(),
        )
        .add_systems(OnExit(GameState::GameOver), cleanup_ui)
        // Buttons are active only on UI screens.
        .add_systems(Update, button_system)
        // Gameplay.
        .add_systems(
            Update,
            (
                player_movement,
                enemy_movement,
                bullet_movement,
                shoot,
                update_hud,
                check_win,
                update_background_size,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    score.0 = 0;

    let (win_w, win_h) = window_query
        .single()
        .map(|w| (w.width(), w.height()))
        .unwrap_or((1280.0, 720.0));

    // Background for the whole match. Keep it behind Player/Enemy/Bullet.
    let game_background: Handle<Image> =
        asset_server.load("ui/hub_background.png");

    commands.spawn((
        GameEntity,
        GameBackground,
        Sprite {
            image: game_background,
            custom_size: Some(Vec2::new(win_w, win_h)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // Map the PNG assets to the in-game Player and Enemy entities.
    let player_image: Handle<Image> = asset_server.load("player/player.png");
    let enemy_image: Handle<Image> = asset_server.load("enemy/enemy.png");

    commands.spawn((
        GameEntity,
        Player { hp: 10 },
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::new(110.0, 110.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let mut rng = rand::rng();

    for _ in 0..50 {
        let x = rng.random_range(-600.0..600.0);
        let y = rng.random_range(-400.0..400.0);

        commands.spawn((
            GameEntity,
            Enemy { hp: 2 },
            Sprite {
                image: enemy_image.clone(),
                custom_size: Some(Vec2::new(70.0, 56.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // Play background music (CandyWin.mp3) on loop
    commands.spawn((
        GameEntity,
        AudioPlayer::new(asset_server.load("audio/CandyWin.mp3")),
        PlaybackSettings::LOOP,
    ));
}

fn update_background_size(
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
    mut bg_query: Query<&mut Sprite, With<GameBackground>>,
) {
    if let Ok(window) = window_query.single() {
        for mut sprite in &mut bg_query {
            sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
        }
    }
}

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn cleanup_ui(
    mut commands: Commands,
    roots: Query<Entity, With<UiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

fn check_win(
    enemies: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemies.is_empty() {
        next_state.set(GameState::Win);
    }
}
