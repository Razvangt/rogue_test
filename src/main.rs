use bevy::{DefaultPlugins, prelude::*, sprite::{SpriteSheetBundle, TextureAtlas}};
use bevy::prelude::{Query, TextureAtlasSprite};
use bevy::time::Time;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::*;

use user_input::*;

mod user_input;
mod animation;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(EguiPlugin)
		.add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin)
		.add_plugin(InputManagerPlugin::<user_input::PlayerInput>::default())
		.add_startup_system(setup)
		.add_startup_system(spawn_player)
		.add_system(animate_sprite)
		.add_system(move_player)
		.run();
}


#[derive(Component)]
struct Player;

fn setup(
	mut commands: Commands,
) {
	// Camera Spawn
	commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

}

const MOVE_SPEED: f32 = 100.;

fn move_player(
	mut player: Query<&mut Transform, With<Player>>,
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
) {
	let mut player = player.single_mut();
	player.translation += MOVE_SPEED * time.delta_seconds();
}


