use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};
use bevy_tnua::{builtins::*, prelude::*};
use bevy_tnua_avian3d::prelude::*;

pub fn player_plugin(app: &mut App) {
	app.add_plugins((
		TnuaControllerPlugin::<PlayerControlScheme>::new(FixedUpdate),
		TnuaAvian3dPlugin::new(FixedUpdate),
	))
	.add_systems(
		Update,
		(
			rotate_player,
			update_player_controller.in_set(TnuaUserControlsSystems),
		),
	);
}

#[derive(Debug, Default, Clone, Copy, SceneComponent)]
pub struct Player;

impl Player {
	pub fn scene() -> impl Scene {
		const HEIGHT: f32 = 1.80;
		const EYE_POSITION: Transform = Transform::from_translation(Vec3::new(
			0.0,
			(HEIGHT - 0.10) / 2.0,
			0.0,
		));

		const CAPSULE_RADIUS: f32 = 0.20;
		const CAPSULE_LENGTH: f32 = HEIGHT - 2.0 * CAPSULE_RADIUS;

		bsn! {
			// Physic
			RigidBody::Dynamic
			LockedAxes::ROTATION_LOCKED
			Collider::capsule(CAPSULE_RADIUS, CAPSULE_LENGTH)

			// Camera
			Children[
				EYE_POSITION
				Camera3d
				--
				EYE_POSITION
				PointLight {
					intensity: light_consts::lumens::LUMENS_PER_LED_WATTS,
					range: 2.0,
					radius: 1.0,
				}
			]

			// Model
			Mesh3d(
				asset_value(Capsule3d::new(CAPSULE_RADIUS, HEIGHT * 0.10))
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
						float_height: HEIGHT / 2.0 + 0.001,
						speed: 2.0,
						..default()
					},
				},
			))
			TnuaAvian3dSensorShape(Collider::cylinder(CAPSULE_RADIUS - 0.02, 0.0))
		}
	}
}

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
enum PlayerControlScheme {}

struct PlayerCamera {
	horizontal: f32,
	vertical: f32,
}

fn rotate_camera(
	time: Res<Time>,
	camera: Single<(&mut Transform, &mut Player, &CharacterAccumulatedInput)>,
) {
}

fn rotate_player(
	time: Res<Time>,
	player: Single<(&mut Transform, &mut Player, &CharacterAccumulatedInput)>,
) {
	let delta_time = time.delta_secs();
	let (mut transform, mut player, input) = player.into_inner();

	if input.look != Vec2::ZERO {
		const ORIENTATION_LIMITS: Vec2 = Vec2::new(90.0, 90.0);
		const SENSITIVITY: Vec2 = Vec2::new(2.0, 2.0);
		let delta_angle = input.look * SENSITIVITY * delta_time;
	}

	if input.movement != Vec2::ZERO {
		let delta_factor =
			(player.sight_orientation.x * 4.0).clamp(-180.0, 180.0);
		let delta_angle = delta_time * delta_factor;

		player.sight_orientation.x -= delta_angle;
		transform.rotate_local_y(delta_angle.to_radians());
	}
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

	let desired_motion = input.movement.x * transform.right()
		+ input.movement.y * transform.forward();

	controller.initiate_action_feeding();
	controller.basis = TnuaBuiltinWalk {
		desired_motion,
		..default()
	};
}
