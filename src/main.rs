mod assets;
mod camera;
mod color;
mod core;
mod materials;
mod ui;

use bevy::{
    prelude::*,
    window::{EnabledButtons, PresentMode},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                title: String::from("rooft"),
                enabled_buttons: EnabledButtons {
                    minimize: false,
                    maximize: false,
                    close: false,
                },
                decorations: false,
                titlebar_shown: false,
                titlebar_transparent: true,
                titlebar_show_title: false,
                titlebar_show_buttons: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(assets::plugin)
        .add_plugins(materials::plugin)
        .add_plugins(core::plugin)
        .add_plugins(camera::plugin)
        .add_plugins(ui::plugin)
        .run()
}
