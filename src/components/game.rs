use bevy::prelude::*;
use bevy_enhanced_input::prelude::InputAction;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct PlayerInput;

#[derive(Component)]
pub struct HudText;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct CoinSlot {
    pub index: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Platform {
    pub size: Vec2,
}

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Jump;
