mod character;

use bevy::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub use character::CharacterController;

pub fn control_plugin(app: &mut App) {
	app.add_plugins((
		TnuaAvian3dPlugin::new(FixedUpdate),
		character::character_control_plugin,
	));
}
