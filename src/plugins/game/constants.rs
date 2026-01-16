use bevy::prelude::{Color, Vec2};

pub const PLAYER_SPEED: f32 = 240.0;
pub const JUMP_SPEED: f32 = 520.0;
pub const PLAYER_SIZE: Vec2 = Vec2::splat(64.0);

pub const PLATFORM_SIZE: Vec2 = Vec2::new(520.0, 28.0);
pub const SMALL_PLATFORM_SIZE: Vec2 = Vec2::new(180.0, 20.0);

pub const COIN_SLOTS: usize = 3;
pub const FERRIS_TEXTURE_SIZE: Vec2 = Vec2::new(460.0, 307.0);
pub const COIN_HEIGHT: f32 = 44.0;
pub const COIN_SIZE: Vec2 = Vec2::new(
    COIN_HEIGHT * (FERRIS_TEXTURE_SIZE.x / FERRIS_TEXTURE_SIZE.y),
    COIN_HEIGHT,
);
pub const COIN_RADIUS: f32 = 20.0;
pub const COIN_ICON_HEIGHT: f32 = 56.0;
pub const COIN_ICON_SIZE: Vec2 = Vec2::new(
    COIN_ICON_HEIGHT * (FERRIS_TEXTURE_SIZE.x / FERRIS_TEXTURE_SIZE.y),
    COIN_ICON_HEIGHT,
);

pub const COIN_SLOT_EMPTY_COLOR: Color = Color::srgb(0.35, 0.35, 0.35);
pub const COIN_SLOT_FILLED_COLOR: Color = Color::WHITE;
pub const HUD_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
