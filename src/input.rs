use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::player::Player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnhancedInputPlugin)
			.add_input_context::<Player>()
			.add_observer(accumulate_movement)
			.add_observer(accumulate_look)
			.add_observer(accumulate_jump)
			.add_systems(PreUpdate, clear_accumulated_input);
	}
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct CharacterAccumulatedInput {
	pub movement: Vec2,
	pub look: Vec2,
	pub jump: bool,
}

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Movement;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Look;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Jump;

pub fn character_input_bundle() -> impl Bundle {
	(
		CharacterAccumulatedInput::default(),
		actions!(Player[
			(
				Action::<Movement>::new(),
				Bindings::spawn((
					Cardinal::wasd_keys(),
					Axial::left_stick(),
				)),
			),
			(
				Action::<Look>::new(),
				Bindings::spawn((
					Spawn((Binding::mouse_motion(), Negate::all())),
					Axial::right_stick(),
				)),
			),
			(
				Action::<Jump>::new(),
				bindings![KeyCode::Space, GamepadButton::South],
			),
		]),
	)
}

fn accumulate_movement(
	movement: On<Fire<Movement>>,
	mut inputs: Query<&mut CharacterAccumulatedInput>,
) {
	let mut inputs = inputs.get_mut(movement.context).unwrap();
	inputs.movement = movement.value;
}

fn accumulate_look(
	look: On<Fire<Look>>,
	mut inputs: Query<&mut CharacterAccumulatedInput>,
) {
	let mut inputs = inputs.get_mut(look.context).unwrap();
	inputs.look = look.value;
}

fn accumulate_jump(
	jump: On<Fire<Jump>>,
	mut inputs: Query<&mut CharacterAccumulatedInput>,
) {
	let mut inputs = inputs.get_mut(jump.context).unwrap();
	inputs.jump = jump.value;
}

fn clear_accumulated_input(inputs: Query<&mut CharacterAccumulatedInput>) {
	for mut inputs in inputs {
		*inputs = CharacterAccumulatedInput::default()
	}
}
