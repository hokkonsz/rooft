// use crate::color;
use bevy::prelude::*;

use crate::{
    color,
    ui::left_panel::{OnHideLeftPanel, OnShowLeftPanel},
};

#[derive(Component)]
pub struct LeftBar;

pub fn on_show_left_panel(
    _trigger: Trigger<OnShowLeftPanel>,
    mut left_bar: Single<&mut BackgroundColor, With<LeftBar>>,
) {
    **left_bar = BackgroundColor(color::BLACK34);
}

pub fn on_hide_left_panel(
    _trigger: Trigger<OnHideLeftPanel>,
    mut left_bar: Single<&mut BackgroundColor, With<LeftBar>>,
) {
    **left_bar = BackgroundColor(color::BLACK30);
}
