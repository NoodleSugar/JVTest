use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};
use bevy_tnua::{builtins::*, prelude::*};
use bevy_tnua_avian3d::prelude::*;

use crate::input::{CharacterAccumulatedInput, character_input_scene};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			crate::input::InputPlugin,
			TnuaControllerPlugin::<PlayerControlScheme>::new(FixedUpdate),
			TnuaAvian3dPlugin::new(FixedUpdate),
		))
		.add_systems(
			Update,
			(
				(rotate_player_camera, rotate_player).chain(),
				update_player_controller.in_set(TnuaUserControlsSystems),
			),
		);
	}
}

#[derive(Debug, Default, Clone, Copy, SceneComponent)]
pub struct Player {
	pub sight_orientation: Vec2,
}

impl Player {
	pub const HEIGHT: f32 = 1.80;
	pub const RADIUS: f32 = 0.20;

	pub const RELATIVE_HEAD_POSITION: Transform = Transform::from_translation(
		Vec3::new(0.0, Player::HEIGHT / 2.0 * 0.8, 0.0),
	);

	pub fn scene() -> impl Scene {
		bsn! {
			// Physic
			RigidBody::Dynamic
			LockedAxes::ROTATION_LOCKED
			Collider::capsule(Self::RADIUS, Self::HEIGHT)
			Children[
				Visibility::default()
				Self::RELATIVE_HEAD_POSITION
				Camera3d::default()
			]

			// Model
			Mesh3d(
				asset_value(Capsule3d::new(Self::RADIUS, Self::HEIGHT * 0.80))
			)
			MeshMaterial3d<StandardMaterial>(
				asset_value(StandardMaterial::from_color(RED))
			)


			// Input
			@character_input_scene()

			// Controller
			TnuaController<PlayerControlScheme>
			TnuaConfig<PlayerControlScheme>(asset_value(
				PlayerControlSchemeConfig {
					basis: TnuaBuiltinWalkConfig {
						float_height: (Player::HEIGHT) / 2.0
							+ Self::RADIUS
							+ 0.001,
						speed: 2.0,
						..default()
					},
					jump: TnuaBuiltinJumpConfig {
						height: 0.4,
						fall_extra_gravity: 0.0,
						shorten_extra_gravity: 0.0,
						..default()
					},
				},
			))
			TnuaAvian3dSensorShape(Collider::cylinder(
				Self::RADIUS - 0.02,
				0.0,
			))
		}
	}
}

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
enum PlayerControlScheme {
	Jump(TnuaBuiltinJump),
}

fn rotate_player(
	time: Res<Time>,
	player: Single<(&mut Transform, &mut Player, &CharacterAccumulatedInput)>,
) {
	let (mut transform, mut player, input) = player.into_inner();

	if input.movement == Vec2::ZERO {
		return;
	}

	let delta_time = time.delta_secs();
	let delta_factor = (player.sight_orientation.x * 4.0).clamp(-180.0, 180.0);
	let delta_angle = delta_time * delta_factor;

	player.sight_orientation.x -= delta_angle;
	transform.rotate_local_y(delta_angle.to_radians());
}

fn rotate_player_camera(
	time: Res<Time>,
	player: Single<(&mut Player, &CharacterAccumulatedInput)>,
	mut camera: Single<&mut Transform, With<Camera3d>>,
) {
	const ORIENTATION_LIMITS: Vec2 = Vec2::new(90.0, 90.0);
	const SENSITIVITY: Vec2 = Vec2::new(2.0, 2.0);

	let delta_time = time.delta_secs();

	let (mut player, accumulated_input) = player.into_inner();
	let delta_angle = accumulated_input.look * SENSITIVITY;

	player.sight_orientation = (player.sight_orientation
		+ delta_time * delta_angle)
		.clamp(-ORIENTATION_LIMITS, ORIENTATION_LIMITS);
	camera.rotation =
		Quat::from_rotation_y(player.sight_orientation.x.to_radians())
			* Quat::from_rotation_x(player.sight_orientation.y.to_radians());
}

fn update_player_controller(
	mut controller: Query<(
		&Transform,
		&CharacterAccumulatedInput,
		&mut TnuaController<PlayerControlScheme>,
	)>,
) {
	let Ok((transform, input, mut controller)) = controller.single_mut() else {
		return;
	};

	let desired_motion = input.movement.x * transform.local_x()
		- input.movement.y * transform.local_z();

	controller.initiate_action_feeding();
	controller.basis = TnuaBuiltinWalk {
		desired_motion,
		..default()
	};
	if input.jump {
		controller.action(PlayerControlScheme::Jump(default()));
	}
}
