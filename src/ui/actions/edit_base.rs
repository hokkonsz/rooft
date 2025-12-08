use bevy::prelude::*;

use crate::{
    assets::AppAssets,
    color,
    core::{
        Axis3d,
        actions::{
            ActionQue, ActionState,
            edit_base::{EditBaseContext, EditBaseState},
        },
        inputs::InputMode,
    },
    ui::{
        BAR_SIZE,
        bundles::button_simple,
        left_panel::{LeftPanel, ResizeLeftPanel},
    },
};

const GAP: f32 = 15.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_info, button).run_if(in_state(ActionState::EditBase)),
    )
    .add_systems(OnEnter(ActionState::EditBase), show)
    .add_systems(OnExit(ActionState::EditBase), hide)
    .add_observer(on_resize_left_panel);
}

#[derive(Component)]
struct EditBaseInfo;

#[derive(Component)]
struct EditBasePanel;

fn show(mut commands: Commands, assets: Res<AppAssets>) {
    // Display Panel
    commands
        .spawn((
            EditBasePanel,
            Node {
                width: Val::Px(100.),
                height: Val::Px(50.),
                right: Val::Px(GAP),
                bottom: Val::Px(GAP),
                box_sizing: BoxSizing::ContentBox,
                border: UiRect::all(Val::Px(10.)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Start,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(color::BLACK34),
            BorderColor::all(color::BLACK34),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::from("Finish editing"),
                TextFont {
                    font: assets.fonts.iosevka.regular.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
            ));

            // Button
            panel.spawn(button_simple(
                "Confirm",
                assets.fonts.iosevka.regular.clone(),
                50.,
                ConfirmButton,
            ));
        });

    // Display Info
    commands.spawn((
        EditBaseInfo,
        Node {
            left: Val::Px(BAR_SIZE + LeftPanel::WIDTH + GAP),
            bottom: Val::Px(GAP),
            width: Val::Px(600.),
            height: Val::Px(30.),
            position_type: PositionType::Absolute,
            ..default()
        },
        Text::from("Edit Base Info"),
        TextFont {
            font: assets.fonts.iosevka.bold_italic.clone(),
            font_size: 12.,
            ..default()
        },
        TextColor(color::WHITE150),
    ));
}

fn hide(
    mut commands: Commands,
    edit_base_panel: Single<Entity, With<EditBasePanel>>,
    edit_base_info: Single<Entity, With<EditBaseInfo>>,
) {
    commands.entity(*edit_base_panel).despawn();
    commands.entity(*edit_base_info).despawn();
}

#[derive(Component)]
struct ConfirmButton;

fn button(
    buttons: Single<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ConfirmButton>),
    >,
    mut action_que: ResMut<ActionQue>,
) {
    let (interaction, mut bg, mut bc) = buttons.into_inner();

    match interaction {
        Interaction::Pressed => {
            *bg = BackgroundColor(color::BLACK44);
            *bc = BorderColor::all(color::BLACK44);

            action_que.next();
        }
        Interaction::Hovered => {
            *bg = BackgroundColor(color::BLACK38);
            *bc = BorderColor::all(color::BLACK38);
        }
        Interaction::None => {
            *bg = BackgroundColor(color::BLACK28);
            *bc = BorderColor::all(color::BLACK28);
        }
    }
}

fn on_resize_left_panel(
    on_resize_left_panel: On<ResizeLeftPanel>,
    mut edit_base_info: Single<&mut Node, With<EditBaseInfo>>,
) {
    edit_base_info.left = Val::Px(on_resize_left_panel.event().0 + BAR_SIZE + GAP)
}

fn update_info(
    edit_base_context: Res<EditBaseContext>,
    edit_base_state: Res<State<EditBaseState>>,
    mut edit_base_info: Single<&mut Text, With<EditBaseInfo>>,
) {
    if !edit_base_state.is_changed() && !edit_base_context.is_changed() {
        return;
    }

    match edit_base_state.get() {
        EditBaseState::PickingNode => {
            ***edit_base_info = format!("Pick a node (left mouse button)\n");
        }
        EditBaseState::PickingAxis => {
            let index = if let Some(index) = edit_base_context.node_index() {
                index.to_string()
            } else {
                String::from("-")
            };

            let x = edit_base_context.original_pos().x;
            let z = edit_base_context.original_pos().z;

            ***edit_base_info = format!(
                concat!(
                    "Pick an axis (left mouse button) or cancel node selection (escape/enter)\n",
                    "Node #{} [ X: {} mm , Z: {} mm ]",
                ),
                index, x, z
            );
        }
        EditBaseState::Reposition => {
            let index = if let Some(index) = edit_base_context.node_index() {
                index.to_string()
            } else {
                String::from("-")
            };

            let input_mode = match edit_base_context.input_mode() {
                InputMode::Mouse => "mouse",
                InputMode::Keyboard => "keyboard",
            };

            let (axis, new_pos, old_pos);
            match edit_base_context.axis() {
                Some(Axis3d::X) => {
                    axis = "X";
                    old_pos = edit_base_context.original_pos().x;
                    new_pos = edit_base_context.new_pos().x;
                }
                Some(Axis3d::Z) => {
                    axis = "Z";
                    old_pos = edit_base_context.original_pos().z;
                    new_pos = edit_base_context.new_pos().z;
                }
                _ => {
                    axis = "-";
                    old_pos = 0.;
                    new_pos = 0.;
                }
            }

            ***edit_base_info = format!(
                concat!(
                    "Reposition node on axis {}, cancel (escape) or confirm new position (enter)\n",
                    "Node #{} [ Z: {} mm -> {} mm ] ({} mode)",
                ),
                axis, index, old_pos, new_pos, input_mode
            );
        }
    }
}
