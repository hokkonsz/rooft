use crate::{
    assets::AppAssets,
    color,
    core::actions::{ActionList, ActionState, Actions},
    ui::{
        BAR_SIZE, LIST_ELEM_BORDER, LIST_ELEM_HEIGHT, LIST_ELEM_MARGIN, bundles::elem_button,
        left_panel::LeftPanel,
    },
};

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(OnEnter(ActionState::Wait), hide)
        .add_systems(OnEnter(ActionState::Selector), display_selector)
        .add_systems(Update, (select, toggle_panel).chain())
        //..
        ;
}

fn setup(mut commands: Commands) {
    commands.spawn((
        ActionsPanel,
        Node {
            width: Val::Px(10.),
            height: Val::Px(10.),
            box_sizing: BoxSizing::ContentBox,
            border: UiRect::all(Val::Px(10.)),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_items: JustifyItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Visibility::Hidden,
        BackgroundColor(color::BLACK34),
        BorderColor(color::BLACK34),
        BorderRadius::all(Val::Px(10.)),
    ));
}

fn hide(mut panel_visibility: Single<&mut Visibility, With<ActionsPanel>>) {
    **panel_visibility = Visibility::Hidden;
}

fn display_selector(
    mut commands: Commands,
    panel: Single<(Entity, &mut Node, &mut Visibility), With<ActionsPanel>>,
    window: Single<&Window>,
    assets: Res<AppAssets>,
) {
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (entity, mut node, mut visibility) = panel.into_inner();

    commands.entity(entity).despawn_related::<Children>();

    *visibility = Visibility::Visible;
    node.left = Val::Px(cursor_position.x);
    node.top = Val::Px(cursor_position.y);

    commands.entity(entity).with_children(|panel| {
        panel.spawn(elem_button(
            "Resize Base",
            assets.fonts.iosevka.regular.clone(),
            Actions::ResizeBaze,
        ));
    });
}

#[derive(Component)]
pub struct ActionsPanel;

fn toggle_panel(
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

fn select(
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
    // mut controller: ResMut<ActionController>,
) {
    // let Some(entity) = controller.id else {
    //     return;
    // };

    // for (interact, actions, mut bg, mut bc) in buttons.iter_mut() {
    //     button_interact(interact, &mut bg, &mut bc, || match actions {
    //         Actions::AddBase => {
    //             commands.entity(entity).despawn_related::<Children>();

    //             // commands.entity(entity).add_children(|action_panel|
    //             //     action_panel.spawn(
    //             //     (
    //             //     Node {
    //             //         width: Val::Px(150.),
    //             //         height: Val::Px(150.),
    //             //         box_sizing: BoxSizing::ContentBox,
    //             //         left,
    //             //         top,
    //             //         border: UiRect::all(Val::Px(10.)),
    //             //         position_type: PositionType::Absolute,
    //             //         align_items: AlignItems::Center,
    //             //         align_content: AlignContent::Center,
    //             //         justify_items: JustifyItems::Start,
    //             //         justify_content: JustifyContent::Start,
    //             //         flex_direction: FlexDirection::Column,
    //             //         ..default()
    //             //     },
    //             //     BackgroundColor(color::BLACK34),
    //             //     BorderColor(color::BLACK34),
    //             //     BorderRadius::all(Val::Px(10.)),
    //             // )));

    //             // commands.();

    //             // commands.spawn(input_panel(
    //             //     panel_node.left,
    //             //     panel_node.top,
    //             //     "Add Base",
    //             //     assets.fonts.iosevka.regular.clone(),
    //             //     vec![(String::from("X"), 15000.), (String::from("Y"), 10000.)],
    //             //     Test1,
    //             // ));
    //         }
    //         Actions::ResizeBaze => {
    //             commands.spawn(input_panel(
    //                 panel_node.left,
    //                 panel_node.top,
    //                 "Resize Base",
    //                 assets.fonts.iosevka.regular.clone(),
    //                 vec![(String::from("X"), 5000.), (String::from("Y"), 10000.)],
    //                 Test1,
    //             ));
    //         }
    //     });
    // }
}

fn button_interact(
    interact: &Interaction,
    background_color: &mut BackgroundColor,
    border_color: &mut BorderColor,
    on_press: impl FnOnce() -> (),
) {
    match interact {
        Interaction::Pressed => on_press(),
        Interaction::Hovered => {
            *background_color = BackgroundColor(color::BLACK44);
            *border_color = BorderColor(color::BLACK44);
        }
        Interaction::None => {
            *background_color = BackgroundColor(color::BLACK30);
            *border_color = BorderColor(color::BLACK30);
        }
    }
}
