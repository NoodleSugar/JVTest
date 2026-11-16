//! Temporary module! Waiting for Bevy capability to load bsn from resource files.

pub mod level;

use avian3d::prelude::*;
use bevy::{
	color::palettes::css::{DARK_GREY, GREY},
	prelude::*,
};

pub fn wall(begin: Vec2, end: Vec2) -> impl Scene {
	const HEIGHT: f32 = 3.0;
	let diff = end - begin;
	let middle = (begin + end) / 2.0;
	let scale = Vec3::ONE.with_x(diff.length());

	bsn! {
		Transform {
			translation: Vec3::new(middle.x, HEIGHT / 2.0, middle.y),
			scale
		}

		RigidBody::Static
		Collider::cuboid(1.0, HEIGHT, 0.1)

		Mesh3d(
			asset_value(Cuboid::new(1.0, HEIGHT, 0.1))
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(GREY))
		)
	}
}

pub fn ground() -> impl Scene {
	bsn! {
		RigidBody::Static
		Collider::cuboid(1.0, 0.0, 1.0)

		Mesh3d(
			asset_value(Plane3d::new(Vec3::Y, 0.5 * Vec2::ONE))
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(DARK_GREY))
		)
	}
}
