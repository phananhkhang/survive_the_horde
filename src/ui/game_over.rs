use crate::ui::menu::{MenuButton, RetryButton};
use crate::{Score, UiRoot};
use bevy::prelude::*;

pub fn setup_game_over(
    mut commands: Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    let background: Handle<Image> = asset_server.load("ui/game_over_background.png");

    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|root| {
            // Full-screen game over background image.
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode::new(background.clone()),
            ));

            // Overlay layer for text readability.
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.14, 0.02, 0.02, 0.45)),
            ));

            // Centered content layout.
            root.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            },))
            .with_children(|ui| {
                ui.spawn((
                    Text::new("YOU LOST"),
                    TextFont {
                        font_size: FontSize::Px(72.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow {
                        offset: Vec2::new(3.0, 3.0),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.6),
                    },
                ));

                ui.spawn((
                    Text::new("The monsters have overwhelmed you."),
                    TextFont {
                        font_size: FontSize::Px(30.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow {
                        offset: Vec2::new(2.0, 2.0),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                    },
                ));

                ui.spawn((
                    Text::new(format!("SCORE {}", score.0)),
                    TextFont {
                        font_size: FontSize::Px(30.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow {
                        offset: Vec2::new(2.0, 2.0),
                        color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                    },
                ));

                ui.spawn((
                    Button,
                    RetryButton,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(68.0),
                        margin: UiRect::top(Val::Px(24.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.45, 0.12, 0.12, 0.95)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("RETRY"),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                ui.spawn((
                    Button,
                    MenuButton,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(68.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.10, 0.10, 0.14, 0.95)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("MENU"),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });
}
