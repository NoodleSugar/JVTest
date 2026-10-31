pub mod input;
pub mod player;

#[cfg(feature = "dev")]
mod debug;

use avian3d::prelude::*;
use bevy::{
	color::palettes::css::{BLUE, GREEN, WHITE},
	prelude::*,
	window::{CursorGrabMode, CursorOptions},
};

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
	commands.spawn_scene(bsn! {
		Transform::from_translation(Vec3::NEG_Z * 1.5 + Vec3::Y * 0.5)

		RigidBody::Static
		Collider::sphere(0.5)

		Mesh3d(
			asset_value(Sphere::default())
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(BLUE))
		)
	});
	commands.spawn_scene(bsn! {
		RigidBody::Static
		Collider::cuboid(10.0, 0.0, 10.0)

		Mesh3d(
			asset_value(Plane3d::new(Vec3::Y, 5.0 * Vec2::ONE))
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(GREEN))
		)
	});
	commands.spawn_scene(bsn! {
		PointLight
		Transform::from_xyz(3.0, -3.0, 5.0)
	});
}
