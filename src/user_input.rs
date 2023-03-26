use bevy::prelude::KeyCode;
use leafwing_input_manager::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub(crate) enum PlayerInput {
	Up,
	Down,
	Right,
	Left
}

impl PlayerInput {
	pub fn player_one() -> InputMap<PlayerInput> {
		let mut map = InputMap::default();
		map.insert_multiple([
			// Left
			(KeyCode::A, PlayerInput::Left),
			(KeyCode::Left, PlayerInput::Left),
			//Right
			(KeyCode::D, PlayerInput::Right),
			(KeyCode::Right, PlayerInput::Right),
			//Up
			(KeyCode::W, PlayerInput::Up),
			(KeyCode::Up, PlayerInput::Up),
			// Down
			(KeyCode::Down, PlayerInput::Down),
			(KeyCode::S, PlayerInput::Down),
		]);
		map
	}
}