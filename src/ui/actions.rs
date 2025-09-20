use crate::{
    assets::AppAssets,
    color,
    core::{
        ActionList, Actions,
        base::{OnResizeBase, OnSpawnBase},
    },
    ui::{
        BAR_SIZE, LIST_ELEM_BORDER, LIST_ELEM_HEIGHT, LIST_ELEM_MARGIN,
        bundles::{elem_button, input_panel},
        left_panel::LeftPanel,
    },
};

use bevy::prelude::*;

#[derive(Component)]
pub struct ActionsPanel;

pub fn toggle_panel(
    mut commands: Commands,
    assets: Res<AppAssets>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    actions: Res<ActionList>,
    actions_panel: Query<Entity, With<ActionsPanel>>,
    left_panel: Single<&Node, With<LeftPanel>>,
    window: Single<&Window>,
) {
    if !mouse_input.just_pressed(MouseButton::Right)
        && !mouse_input.just_pressed(MouseButton::Left)
        && !keyboard_input.just_pressed(KeyCode::Escape)
    {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    if let Some(ap_entity) = actions_panel.into_iter().next() {
        commands.entity(ap_entity).despawn();
    } else if !mouse_input.just_pressed(MouseButton::Left)
        && !keyboard_input.just_pressed(KeyCode::Escape)
    {
        let Val::Px(width) = left_panel.into_inner().width else {
            return;
        };

        if cursor_pos.x > width + BAR_SIZE && cursor_pos.y > BAR_SIZE {
            if !actions.list.is_empty() {
                commands
                    .spawn((
                        ActionsPanel,
                        Node {
                            width: Val::Px(150.),
                            height: Val::Px(
                                actions.list.len() as f32
                                    * (LIST_ELEM_HEIGHT
                                        + (LIST_ELEM_BORDER + LIST_ELEM_MARGIN) * 2.)
                                    - LIST_ELEM_MARGIN,
                            ),
                            box_sizing: BoxSizing::ContentBox,
                            left: Val::Px(cursor_pos.x),
                            top: Val::Px(cursor_pos.y),
                            border: UiRect::all(Val::Px(10.)),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            justify_items: JustifyItems::Start,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(color::BLACK34),
                        BorderColor(color::BLACK34),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_children(|panel| {
                        for action in actions.list.iter() {
                            panel.spawn(elem_button(
                                action.to_string(),
                                assets.fonts.iosevka.regular.clone(),
                                action.clone(),
                            ));
                        }
                    });
            } else {
                commands
                    .spawn((
                        ActionsPanel,
                        Node {
                            width: Val::Px(150.),
                            height: Val::Px(LIST_ELEM_HEIGHT + LIST_ELEM_BORDER * 2.),
                            box_sizing: BoxSizing::ContentBox,
                            left: Val::Px(cursor_pos.x),
                            top: Val::Px(cursor_pos.y),
                            border: UiRect::all(Val::Px(10.)),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            justify_items: JustifyItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(color::BLACK34),
                        BorderColor(color::BLACK34),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_child((
                        Text::from("No actions"),
                        TextFont {
                            font: assets.fonts.iosevka.regular.clone(),
                            font_size: 12.,
                            ..default()
                        },
                        TextColor(color::BLACK68),
                    ));
            }
        }
    }
}

#[derive(Component)]
struct Test1;

#[derive(Component)]
struct Test2;

pub fn select(
    mut buttons: Query<
        (
            &Interaction,
            &Actions,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Actions>),
    >,
    panel_node: Single<&Node, With<ActionsPanel>>,
    mut commands: Commands,
    assets: Res<AppAssets>,
) {
    for (interact, actions, mut bg, mut bc) in buttons.iter_mut() {
        match interact {
            Interaction::Pressed => match actions {
                Actions::AddBase => {
                    commands.spawn(input_panel(
                        panel_node.left,
                        panel_node.top,
                        "Add Base",
                        assets.fonts.iosevka.regular.clone(),
                        vec![(String::from("X"), 15000.), (String::from("Y"), 10000.)],
                        Test1,
                    ));
                }
                Actions::ResizeBaze => {
                    commands.spawn(input_panel(
                        panel_node.left,
                        panel_node.top,
                        "Resize Base",
                        assets.fonts.iosevka.regular.clone(),
                        vec![(String::from("X"), 5000.), (String::from("Y"), 10000.)],
                        Test1,
                    ));
                }
            },
            Interaction::Hovered => {
                *bg = BackgroundColor(color::BLACK44);
                *bc = BorderColor(color::BLACK44);
            }
            Interaction::None => {
                *bg = BackgroundColor(color::BLACK30);
                *bc = BorderColor(color::BLACK30);
            }
        }
    }
}
