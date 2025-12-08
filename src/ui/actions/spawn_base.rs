use bevy::prelude::*;

use crate::{
    assets::AppAssets,
    color,
    core::actions::{
        ActionQue, ActionState,
        spawn_base::{BaseShape, SpawnBase},
    },
    ui::{BAR_SIZE, bundles::button, left_panel::LeftPanel},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update, buttons).run_if(in_state(ActionState::SpawnBase)),
    )
    .add_systems(OnEnter(ActionState::SpawnBase), show)
    .add_systems(OnExit(ActionState::SpawnBase), hide);
}

#[derive(Component)]
struct SpawnBasePanel;

fn show(mut commands: Commands, assets: Res<AppAssets>) {
    // Display Panel
    commands
        .spawn((
            SpawnBasePanel,
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
            BorderColor::all(color::BLACK34),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Node {
                    height: Val::Px(30.),
                    ..default()
                },
                Text::from("Select base shape"),
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

fn hide(mut commands: Commands, panel: Single<Entity, With<SpawnBasePanel>>) {
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
                *bc = BorderColor::all(color::BLACK44);

                commands.trigger(SpawnBase(*shape));
                action_que.next();

                break;
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
}

fn update(
    left_panel: Single<&Node, (With<LeftPanel>, Without<SpawnBasePanel>)>,
    mut reshape_panel: Single<&mut Node, With<SpawnBasePanel>>,
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
