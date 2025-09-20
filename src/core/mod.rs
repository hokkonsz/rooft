pub mod base;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<ElementList>()
        .init_state::<ActionState>()
        .add_event::<base::OnSpawnBase>()
        .add_systems(Startup, setup)
        .add_observer(base::on_spawn_base)
        .add_observer(base::on_resize_base);
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ActionList {
        list: vec![Actions::AddBase],
    });
}

#[derive(Default, Resource)]
pub struct ActionList {
    pub list: Vec<Actions>,
}

#[derive(Default, Resource)]
pub struct ElementList {
    pub list: Vec<(Entity, String)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum ActionState {
    // Waiting For Valid Input
    #[default]
    Wait,

    // Action Selector
    Selector,

    // Actions
    ResizeBase,
    Reset,

    // Results
    Success,
    Canceled,
}

#[derive(Component, Clone)]
pub enum Actions {
    AddBase,
    ResizeBaze,
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::AddBase => write!(f, "Add Base"),
            Actions::ResizeBaze => write!(f, "Resize Base"),
        }
    }
}
