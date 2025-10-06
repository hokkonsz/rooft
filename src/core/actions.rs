use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<ActionState>()
        .add_systems(Startup, setup)
		.add_systems(Update, wait.run_if(in_state(ActionState::Wait)))
        // .insert_resource(MeshPickingSettings{require_markers: false, ray_cast_visibility: RayCastVisibility::Any})
		// .add_systems(OnEnter(Selector), selector)
		// ..
		;
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ActionList {
        list: vec![Actions::AddBase],
    });
}

#[derive(Default, Resource)]
pub struct ActionList {
    pub list: Vec<Actions>,
}

fn wait(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
) {
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
