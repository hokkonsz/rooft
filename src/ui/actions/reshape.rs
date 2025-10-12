use bevy::prelude::*;

use crate::{
    assets::AppAssets,
    color,
    core::{
        actions::{ActionQue, ActionState},
        base::{BaseShape, OnReshapeBase},
    },
    ui::{BAR_SIZE, bundles::button, left_panel::LeftPanel},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update, buttons).run_if(in_state(ActionState::ReshapeBase)),
    )
    .add_systems(OnEnter(ActionState::ReshapeBase), show)
    .add_systems(OnExit(ActionState::ReshapeBase), hide);
}

#[derive(Component)]
struct ReshapePanel;

fn show(mut commands: Commands, assets: Res<AppAssets>) {
    // Display Panel
    commands
        .spawn((
            ReshapePanel,
            Node {
                width: Val::Px(420.),
                height: Val::Px(110.),
                box_sizing: BoxSizing::ContentBox,
                border: UiRect::all(Val::Px(10.)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_content: AlignContent::SpaceEvenly,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::SpaceBetween,
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
                Node {
                    height: Val::Px(30.),
                    ..default()
                },
                Text::from("Select shape"),
                TextFont {
                    font: assets.fonts.iosevka.regular.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
            ));

            // Shapes
            panel
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|shape_list| {
                    shape_list.spawn((
                        BaseShape::Rectangle,
                        button(100., 50., &assets.images.shape_rect, 90., 45., ShapeButton),
                    ));
                    shape_list.spawn((
                        BaseShape::L,
                        button(100., 50., &assets.images.shape_l, 90., 45., ShapeButton),
                    ));
                    shape_list.spawn((
                        BaseShape::N,
                        button(100., 50., &assets.images.shape_n, 90., 45., ShapeButton),
                    ));
                });
        });
}

fn hide(mut commands: Commands, panel: Single<Entity, With<ReshapePanel>>) {
    commands.entity(*panel).despawn();
}

#[derive(Component)]
struct ShapeButton;

fn buttons(
    mut commands: Commands,
    buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &BaseShape,
        ),
        (Changed<Interaction>, With<ShapeButton>),
    >,
    mut action_que: ResMut<ActionQue>,
) {
    for (interaction, mut bg, mut bc, shape) in buttons {
        match interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(color::BLACK44);
                *bc = BorderColor(color::BLACK44);

                commands.trigger(OnReshapeBase(*shape));
                action_que.next();

                break;
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

fn update(
    left_panel: Single<&Node, (With<LeftPanel>, Without<ReshapePanel>)>,
    mut reshape_panel: Single<&mut Node, With<ReshapePanel>>,
    window: Single<&Window>,
) {
    let Val::Px(left_panel_width) = left_panel.width else {
        return;
    };

    let left_side = left_panel_width + BAR_SIZE;
    let center_x = (window.width() - left_side) * 0.5 + left_side;

    if let Val::Px(width) = reshape_panel.width {
        reshape_panel.left = Val::Px(center_x - width * 0.5);
    }

    let center_y = (window.height() - BAR_SIZE) * 0.5 + BAR_SIZE;

    if let Val::Px(height) = reshape_panel.height {
        reshape_panel.top = Val::Px(center_y - height * 0.5);
    }
}
