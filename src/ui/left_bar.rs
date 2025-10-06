use bevy::prelude::*;

use crate::{
    color,
    ui::{
        BAR_SIZE,
        left_panel::{OnHideLeftPanel, OnShowLeftPanel},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_observer(on_show_left_panel)
        .add_observer(on_hide_left_panel)
        //..
        ;
}

fn setup(mut commands: Commands) {
    commands.spawn((
        LeftBar,
        Node {
            width: Val::Px(BAR_SIZE),
            height: Val::Percent(100.),
            top: Val::Px(BAR_SIZE),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            flex_shrink: 0.,
            ..default()
        },
        BackgroundColor(color::BLACK34),
    ));
}

#[derive(Component)]
pub struct LeftBar;

fn on_show_left_panel(
    _trigger: Trigger<OnShowLeftPanel>,
    mut left_bar: Single<&mut BackgroundColor, With<LeftBar>>,
) {
    **left_bar = BackgroundColor(color::BLACK34);
}

fn on_hide_left_panel(
    _trigger: Trigger<OnHideLeftPanel>,
    mut left_bar: Single<&mut BackgroundColor, With<LeftBar>>,
) {
    **left_bar = BackgroundColor(color::BLACK30);
}
