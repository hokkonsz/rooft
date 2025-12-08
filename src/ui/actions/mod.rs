mod spawn_base;

use bevy::app::App;

pub fn plugin(app: &mut App) {
    app.add_plugins(spawn_base::plugin)
		//..
	;
}
