use bevy::prelude::*;
use bevy_tnua::{builtins::*, prelude::*};
use leafwing_input_manager::prelude::*;

pub fn character_control_plugin(app: &mut App) {
	app.add_plugins((
		InputManagerPlugin::<CharacterAction>::default(),
		TnuaControllerPlugin::<CharacterControlScheme>::new(FixedUpdate),
	));
}

#[derive(SceneComponent, Debug, Default, Clone, Copy)]
#[scene(CharacterControllerProps)]
pub struct CharacterController;

#[derive(Default)]
struct CharacterControllerProps {
	height: f32,
}

impl CharacterController {
	pub fn scene(props: CharacterControllerProps) -> impl Scene {
		bsn! {
			character_input_mapping_keyboard()

			TnuaController<CharacterControlScheme>
			TnuaConfig<CharacterControlScheme>(asset_value(CharacterControlSchemeConfig {
				basis: TnuaBuiltinWalkConfig {
					float_height: props.height / 2.0 + 0.001,
					speed: 2.0,
					..default()
				},
			}))

			avian3d::dynamics::rigid_body::LockedAxes::ROTATION_LOCKED
		}
	}
}

#[derive(Actionlike, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterAction {
	#[actionlike(DualAxis)]
	Move,
	#[actionlike(DualAxis)]
	Look,
}

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
enum CharacterControlScheme {}

fn control_character(
	query: Query<
		(
			&Transform,
			&ActionState<CharacterAction>,
			&mut TnuaController<CharacterControlScheme>,
		),
		With<CharacterController>,
	>,
) {
	for (transform, action_state, controller) in query {
		controller.initiate_action_feeding();

		if let Some(movement) =
			action_state.dual_axis_data(&CharacterAction::Move)
		{
			controller.basis = TnuaBuiltinWalk {
				// Move in the direction the player entered:
				desired_motion: movement.pair.x * transform.right()
					+ movement.pair.y * transform.forward(),

				// Turn the character in the movement direction:
				desired_forward: Dir3::new(action_state.direction_vector())
					.ok(),
			};
		}
	}
}


pub fn character_input_mapping_keyboard() -> InputMap<CharacterAction> {
	InputMap::default()
		.with_dual_axis(CharacterAction::Move, VirtualDPad::wasd())
		.with_dual_axis(CharacterAction::Look, MouseMove::default())
}

// fn control_character(
// 	query: Query<
// 		(
// 			&Transform,
// 			&ActionState<CharacterAction>,
// 			&mut TnuaController<CharacterControlScheme>,
// 		),
// 		With<CharacterController>,
// 	>,
// ) {
// 	for (transform, action_state, controller) in query {
// 		controller.initiate_action_feeding();

// 		if let Some(movevement) =
// 			action_state.dual_axis_data(&CharacterAction::Move)
// 		{
// 			controller.basis = TnuaBuiltinWalk {
// 				// Move in the direction the player entered:
// 				desired_motion: movevement.pair.x * transform.right()
// 					+ movevement.pair.y * transform.forward(),

// 				// Turn the character in the movement direction:
// 				desired_forward: Dir3::new(action_state.direction_vector())
// 					.ok(),
// 			};
// 		}
// 	}
// }
