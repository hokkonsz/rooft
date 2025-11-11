pub mod actions;
pub mod base;
mod picking;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(actions::plugin)
        .add_plugins(picking::plugin)
        .init_resource::<ElementList>()
        // .init_state::<actions::ActionState>()
        // .add_systems(Startup, actions::setup)
        .add_observer(base::on_spawn_base);
}

#[derive(Default, Resource)]
pub struct ElementList {
    pub list: Vec<(Entity, String)>,
}
