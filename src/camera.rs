use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

// #[derive(Default)]
// enum View {
//     #[default]
//     Top,
//     Front,
//     Back,
//     Left,
//     Right,
//     ThirdDimension,
// }
