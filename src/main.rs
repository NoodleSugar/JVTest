extern crate fear_itself_game as game;

use game::game_plugin;

fn main() {
	bevy::app::App::new().add_plugins(game_plugin).run();
}
