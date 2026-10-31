use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{assets, player::Player};

pub fn test_level1() -> impl SceneList {
	bsn_list! {
		PointLight
		Transform::from_xyz(3.0, -5.0, 3.0)
		--
		RigidBody::Static
		Collider::half_space(Vec3::Y)
		--
		@Player

		-- // Grounds
		@assets::ground()
		Transform::from_scale(10.0 * Vec3::X + Vec3::Y + 10.0 * Vec3::Z)

		-- // Walls
		@assets::wall()
		Transform {
			translation: Vec3::new(0.0, 0.0, -1.0),
			scale: {4.0 * Vec3::X + 3.0 * Vec3::Y + Vec3::Z}
		}
	}
}
