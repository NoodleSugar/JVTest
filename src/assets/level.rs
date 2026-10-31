use avian3d::prelude::*;
use bevy::{
	color::palettes::css::{BLUE, GREEN},
	prelude::*,
};

pub fn test_level1() -> impl SceneList {
	bsn_list! {
		Transform::from_translation(Vec3::NEG_Z * 1.5 + Vec3::Y * 0.5)

		RigidBody::Static
		Collider::sphere(0.5)

		Mesh3d(
			asset_value(Sphere::default())
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(BLUE))
		)
		--
		RigidBody::Static
		Collider::cuboid(10.0, 0.0, 10.0)

		Mesh3d(
			asset_value(Plane3d::new(Vec3::Y, 5.0 * Vec2::ONE))
		)
		MeshMaterial3d<StandardMaterial>(
			asset_value(StandardMaterial::from_color(GREEN))
		)
		--
		PointLight
		Transform::from_xyz(3.0, -3.0, 5.0)
	}
}
