use crate::player::Player;
use crate::{Score, UiRoot};
use bevy::prelude::*;

#[derive(Component)]
pub struct HpText;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(76.0),
                    padding: UiRect::axes(Val::Px(28.0), Val::Px(16.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.04, 0.07, 0.10, 0.72)),
            ))
            .with_children(|bar| {
                bar.spawn((
                    HpText,
                    Text::new("HP 10/10"),
                    TextFont {
                        font_size: FontSize::Px(28.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                bar.spawn((
                    ScoreText,
                    Text::new("SCORE 0"),
                    TextFont {
                        font_size: FontSize::Px(28.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

pub fn update_hud(
    player: Query<&Player>,
    score: Res<Score>,
    mut hp_text: Query<&mut Text, With<HpText>>,
    mut score_text: Query<&mut Text, (With<ScoreText>, Without<HpText>)>,
) {
    let hp = player.single().map(|p| p.hp).unwrap_or(0);

    if let Ok(mut text) = hp_text.single_mut() {
        *text = Text::new(format!("HP {}/10", hp));
    }

    if let Ok(mut text) = score_text.single_mut() {
        *text = Text::new(format!("SCORE {}", score.0));
    }
}
