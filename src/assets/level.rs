use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{assets, player::Player};

pub fn test_level1() -> impl SceneList {
	bsn_list! {
		RigidBody::Static
		Collider::half_space(Vec3::Y)
		--
		@Player
		-- // Grounds
		@assets::ground()
		Transform::from_scale(10.0 * Vec3::X + Vec3::Y + 10.0 * Vec3::Z)

		-- // Walls
		@assets::wall(Vec2::new(0.0, -1.0),Vec2::new(6.0, -1.0))
		--
		@assets::wall(Vec2::new(6.0, 0.0),Vec2::new(18.0, 0.0))
		--
	}
}
