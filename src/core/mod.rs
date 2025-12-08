pub mod actions;
pub mod base;
pub mod inputs;
pub mod picking;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<ElementList>()
        .add_plugins(actions::plugin)
        .add_plugins(picking::plugin)
        // .init_state::<actions::ActionState>()
        // .add_systems(Startup, actions::setup)
        ;
}

#[derive(Default, Resource)]
pub struct ElementList {
    pub list: Vec<(Entity, String)>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Axis3d {
    X,
    Y,
    Z,
}
