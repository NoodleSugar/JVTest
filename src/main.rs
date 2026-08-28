use avian3d::prelude::*;
use bevy::{
	color::palettes::css::{BLUE, GREEN, WHITE},
	prelude::*,
	window::{CursorGrabMode, CursorOptions},
};

mod input;
mod player;

fn main() {
	App::new()
		.add_plugins((
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
		.add_systems(Startup, setup)
		.insert_resource(ClearColor(WHITE.into()))
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_scene(bsn!(
		Transform::from_translation(Vec3::NEG_Z * 1.5 + Vec3::Y * 0.5)

		RigidBody::from(RigidBody::Static)
		Collider::sphere(0.5)

		Mesh3d(
			asset_value(Sphere::default())
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(BLUE))
		)
	));
	commands.spawn_scene(bsn!(
		RigidBody::from(RigidBody::Static)
		Collider::cuboid(10.0, 0.0, 10.0)

		Mesh3d(
			asset_value(Plane3d::new(Vec3::Y, 5.0 * Vec2::ONE))
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(GREEN))
		)
	));
	commands
		.spawn((PointLight::default(), Transform::from_xyz(3.0, -3.0, 5.0)));
}
