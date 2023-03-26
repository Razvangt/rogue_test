use bevy::prelude::*;
use std::collections::HashMap;
use bevy::utils::label::DynEq;
use crate::Player;

pub struct RogueAnimationPlugin;

impl Plugin for RogueAnimationPlugin {
	fn build ( &self, app: &mut App ){
		app
			.add_system(animate_sprite)
			.add_system(change_player_animation)
			.init_resource::<Animations>();
	}
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
	MainRunHorizontal,
	MainRunUp,
	MainRunDown,
	MainIdle
}

#[derive(Component, Clone, Copy)]
struct SpriteAnimation {
	len: usize,
	frame_time: f32,
}
impl SpriteAnimation{
	fn new (len: usize, fps: usize) -> SpriteAnimation {
		SpriteAnimation{ len, frame_time: 1./fps as f32}
	}
}
#[derive(Component)]
pub struct FrameTime(pub f32);

#[derive(Bundle)]
pub struct RogueAnimationBundle{
	pub animation : SpriteAnimation,
	frame_time : FrameTime,
}

impl RogueAnimationBundle {
	pub fn new(animation: SpriteAnimation) -> RogueAnimationBundle {
		RogueAnimationBundle {
			animation,
			frame_time: FrameTime(0.0),
		}
	}
}

#[derive(Resource)]
pub struct Animations {
	map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl FromWorld for Animations {
	fn from_world(world: &mut World) -> Self {
		let mut map = Animations {map: HashMap::new()};
		world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {
			let asset_server = world.resource::<AssetServer>();
			// Mask Dude
			let idel_atlas = TextureAtlas::from_grid(
				asset_server.load("textures/mc/idle.png"),
				Vec2::splat(32.),
				11, 1, None, None);
		});

		map
	}
}

impl Animations {
	pub fn add(&mut self, id: Animation, handle: Handle<TextureAtlas>, animation: SpriteAnimation) {
		self.map.insert(id, (handle, animation));
	}
	pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
		self.map.get(&id).cloned()
	}
}
fn animate_sprite(
	mut animations: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
	time: Res<Time>,
) {
	for (mut sprite, animation, mut frame_time) in animations.iter_mut() {
		frame_time.0 += time.delta_seconds();
		if frame_time.0 > animation.frame_time {
			let frames = (frame_time.0 / animation.frame_time) as usize;
			sprite.index += frames;
			if sprite.index >= animation.len {
				sprite.index %= animation.len;
			}
			frame_time.0 -= animation.frame_time;
		}
	}
}

fn change_player_animation(
	mut player: Query<(&Player, &mut Handle<TextureAtlas>, &mut SpriteAnimation, &mut TextureAtlasSprite, &Velocity), With<Player>>,
	animations: Res<Animations>,
){
	let (player, mut atlas, mut animation, mut sprite,  velocity) = player.single_mut();
	if velocity.linvel.x < -0.1 {
		sprite.flip_x = true;
	} else if velocity.linvel.x > 0.1 {
		sprite.flip_x = false;
	}

	let  set : Animation =  if  velocity.

}