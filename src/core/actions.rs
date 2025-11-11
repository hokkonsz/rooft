use std::collections::VecDeque;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<ActionState>()
        .add_systems(Startup, setup)
		.add_systems(Update, update)
		// ..
		;
}

fn setup(mut commands: Commands) {
    let mut que = ActionQue::new();

    que.add(ActionState::SpawnBase);

    commands.insert_resource(que);
}

#[derive(Default, Resource)]
pub struct ActionQue {
    que: VecDeque<ActionState>,
}

impl ActionQue {
    fn new() -> Self {
        Self {
            que: VecDeque::new(),
        }
    }

    pub fn next(&mut self) {
        let _ = self.que.pop_back();
    }

    pub fn add(&mut self, state: ActionState) {
        self.que.push_front(state);
    }
}

fn update(action_que: Res<ActionQue>, mut next_state: ResMut<NextState<ActionState>>) {
    if !action_que.is_changed() {
        return;
    }

    if let Some(action_state) = action_que.que.back() {
        info!("New State: {action_state:?}");
        next_state.set(*action_state);
    } else {
        info!("New State: None");
        next_state.set(ActionState::None);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum ActionState {
    #[default]
    None,

    SpawnBase,
}
