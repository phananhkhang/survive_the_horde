use crate::GameState;
use crate::UiRoot;
use bevy::prelude::*;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct RetryButton;

#[derive(Component)]
pub struct MenuButton;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background: Handle<Image> = asset_server.load("ui/menu_background.png");
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
            // Full-screen background image.
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode::new(background.clone()),
            ));

            // A subtle dark layer keeps the text readable.
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.10)),
            ));

            root.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },))
                .with_children(|ui| {
                    ui.spawn((
                        Text::new("Survive The Horde"),
                        TextFont {
                            font_size: FontSize::Px(72.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextShadow {
                            offset: Vec2::new(3.0, 3.0),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.45),
                        },
                    ));

                    ui.spawn((
                        Text::new("A lone warrior against the horde."),
                        TextFont {
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextShadow {
                            offset: Vec2::new(2.0, 2.0),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.40),
                        },
                    ));

                    spawn_button(ui, "START", ButtonKind::Start);
                });
        });
}

enum ButtonKind {
    Start,
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, kind: ButtonKind) {
    let mut button = parent.spawn((
        Button,
        Node {
            width: Val::Px(240.0),
            height: Val::Px(72.0),
            margin: UiRect::top(Val::Px(18.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.82)),
    ));

    match kind {
        ButtonKind::Start => {
            button.insert(StartButton);
        }
    }

    button.with_children(|button| {
        button.spawn((
            Text::new(label),
            TextFont {
                font_size: FontSize::Px(30.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn button_system(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartButton>,
            Option<&RetryButton>,
            Option<&MenuButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background, start, retry, menu) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Color::srgba(0.25, 0.25, 0.32, 0.95));

                if start.is_some() || retry.is_some() {
                    next_state.set(GameState::Playing);
                } else if menu.is_some() {
                    next_state.set(GameState::Menu);
                }
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgba(0.18, 0.18, 0.24, 0.92));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.82));
            }
        }
    }
}
