use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<ElementList>()
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    let list = vec![Actions::AddBase, Actions::ResizeBaze];
    commands.insert_resource(ActionList { list });
}

#[derive(Default, Resource)]
pub struct ActionList {
    pub list: Vec<Actions>,
}

#[derive(Default, Resource)]
pub struct ElementList {
    pub list: Vec<(Entity, Name)>,
}

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
