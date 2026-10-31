pub mod assets;
pub mod input;
pub mod player;
pub mod world;

#[cfg(feature = "dev")]
mod debug;

use avian3d::prelude::*;
use bevy::{
	color::palettes::css::WHITE,
	prelude::*,
	window::{CursorGrabMode, CursorOptions},
};

use crate::assets::level::test_level1;

pub fn game_plugin(app: &mut App) {
	app.add_plugins(bevy_plugin).add_systems(Startup, setup);

	#[cfg(feature = "dev")]
	app.add_plugins(debug::debug_plugin);
}

fn bevy_plugin(app: &mut App) {
	app.add_plugins((
		DefaultPlugins.set(WindowPlugin {
			primary_cursor_options: Some(CursorOptions {
				grab_mode: CursorGrabMode::Locked,
				visible: false,
				..default()
			}),
			..default()
		}),
		PhysicsPlugins::default(),
		crate::player::PlayerPlugin,
	))
	.insert_resource(ClearColor(WHITE.into()));
}

fn setup(mut commands: Commands) {
	commands.spawn_scene_list(test_level1());
}
