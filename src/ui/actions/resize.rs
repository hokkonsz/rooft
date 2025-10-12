use bevy::prelude::*;
use bevy_ui_text_input::TextInputBuffer;

use crate::{
    assets::AppAssets,
    color,
    core::{
        actions::{ActionQue, ActionState},
        base::OnResizeBase,
    },
    ui::{
        LIST_ELEM_BORDER, LIST_ELEM_HEIGHT, LIST_ELEM_MARGIN,
        bundles::{ChildSpawnerExt, Unit, button_simple},
        utils::UiRectExt,
    },
};

const GAP: f32 = 10.;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update, buttons).run_if(in_state(ActionState::ResizeBase)),
    )
    .add_systems(OnEnter(ActionState::ResizeBase), show)
    .add_systems(OnExit(ActionState::ResizeBase), hide);
}

#[derive(Component)]
struct ResizePanel;

#[derive(Component, Clone, Copy)]
#[repr(usize)]
enum InputFieldName {
    DimensionX,
    DimensionY,
    Count,
}

impl InputFieldName {
    const COUNT: usize = Self::Count as usize;
}

fn show(mut commands: Commands, assets: Res<AppAssets>) {
    // Display Panel
    commands
        .spawn((
            ResizePanel,
            Node {
                width: Val::Px(150.),
                height: Val::Px(100.),
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
            BackgroundColor(color::BLACK34),
            BorderColor(color::BLACK34),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::from("Set size"),
                TextFont {
                    font: assets.fonts.iosevka.regular.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
            ));

            // Input Fields
            panel.input_field(
                &assets.fonts,
                "X",
                InputFieldName::DimensionX,
                Unit::Millimeter,
            );

            panel.input_field(
                &assets.fonts,
                "Y",
                InputFieldName::DimensionY,
                Unit::Millimeter,
            );

            // Buttons
            panel
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Px(LIST_ELEM_HEIGHT),
                    box_sizing: BoxSizing::ContentBox,
                    border: UiRect::all(Val::Px(LIST_ELEM_BORDER)),
                    position_type: PositionType::Relative,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::vertical(Val::Px(LIST_ELEM_MARGIN)),
                    ..default()
                })
                .with_children(|buttons| {
                    buttons.spawn(button_simple(
                        "Confirm",
                        assets.fonts.iosevka.regular.clone(),
                        30.,
                        PanelButton::Confirm,
                    ));

                    buttons.spawn(button_simple(
                        "Cancel",
                        assets.fonts.iosevka.regular.clone(),
                        30.,
                        PanelButton::Cancel,
                    ));
                });
        });
}

fn hide(mut commands: Commands, panel: Single<Entity, With<ResizePanel>>) {
    commands.entity(*panel).despawn();
}

#[derive(Component)]
enum PanelButton {
    Confirm,
    Cancel,
}

fn buttons(
    buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &PanelButton,
        ),
        (Changed<Interaction>, With<PanelButton>),
    >,
    fields: Query<(&InputFieldName, &TextInputBuffer), With<InputFieldName>>,
    mut action_que: ResMut<ActionQue>,
    mut commands: Commands,
) {
    for (interaction, mut bg, mut bc, button) in buttons {
        match interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(color::BLACK44);
                *bc = BorderColor(color::BLACK44);

                match button {
                    PanelButton::Confirm => {
                        let mut values: [f32; InputFieldName::COUNT] = [0.; InputFieldName::COUNT];
                        for (name, text) in fields {
                            values[*name as usize] = text.get_text().parse().unwrap();
                        }

                        commands.trigger(OnResizeBase::from(values));
                        action_que.next();
                    }
                    PanelButton::Cancel => {
                        action_que.next();
                    }
                }
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(color::BLACK38);
                *bc = BorderColor(color::BLACK38);
            }
            Interaction::None => {
                *bg = BackgroundColor(color::BLACK30);
                *bc = BorderColor(color::BLACK30);
            }
        }
    }
}

fn update(mut resize_panel: Single<&mut Node, With<ResizePanel>>, window: Single<&Window>) {
    if let Val::Px(width) = resize_panel.width {
        resize_panel.left =
            Val::Px(window.width() - width - resize_panel.border.px_horizontal() - GAP);
    }

    if let Val::Px(height) = resize_panel.height {
        resize_panel.top =
            Val::Px(window.height() - height - resize_panel.border.px_vertical() - GAP);
    }
}
