mod actions;
mod bundles;
mod left_bar;
mod left_panel;
mod top_bar;

use bevy::prelude::*;
use bevy_ui_text_input::TextInputPlugin;

const BAR_SIZE: f32 = 40.;
const LIST_ELEM_HEIGHT: f32 = 15.;
const LIST_ELEM_MARGIN: f32 = 3.;
const LIST_ELEM_BORDER: f32 = 5.;

pub fn plugin(app: &mut App) {
    app.add_plugins(TextInputPlugin)
        .add_plugins(top_bar::plugin)
        .add_plugins(left_bar::plugin)
        .add_plugins(left_panel::plugin)
        .add_plugins(actions::plugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        ;
}
