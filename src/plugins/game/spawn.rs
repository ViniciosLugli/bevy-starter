use avian2d::prelude::*;
use bevy::{prelude::*, text::FontSmoothing};
use bevy_enhanced_input::prelude::*;

use crate::{
    components::{Coin, CoinSlot, Ground, HudText, Jump, Move, Platform, Player, PlayerInput},
    plugins::fonts::FontAssets,
};

use super::constants::{
    COIN_ICON_SIZE, COIN_RADIUS, COIN_SIZE, COIN_SLOT_EMPTY_COLOR, COIN_SLOTS, HUD_TEXT_COLOR,
    PLATFORM_SIZE, PLAYER_SIZE, SMALL_PLATFORM_SIZE,
};

#[derive(Clone, Copy)]
struct PlatformSpec {
    position: Vec2,
    size: Vec2,
    color: Color,
}

pub fn spawn_player(commands: &mut Commands, assets: &AssetServer) {
    let player_texture = assets.load("textures/bevy.png");
    let player = commands
        .spawn((
            Player,
            PlayerInput,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::rectangle(PLAYER_SIZE.x, PLAYER_SIZE.y),
            LinearVelocity::ZERO,
            // Keep wall sliding smooth when pushing into vertical surfaces.
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO,
            LockedAxes::ROTATION_LOCKED,
            CollidingEntities::default(),
            actions!(PlayerInput[
                (
                    Action::<Move>::new(),
                    ActionSettings {
                        consume_input: false,
                        ..default()
                    },
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Cardinal::arrows(),
                        Axial::left_stick(),
                    )),
                ),
                (
                    Action::<Jump>::new(),
                    bindings![
                        KeyCode::Space,
                        KeyCode::KeyW,
                        KeyCode::ArrowUp,
                        GamepadButton::South
                    ],
                ),
            ]),
        ))
        .id();

    // Visual base layer.
    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0x28, 0x28, 0x28),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        ChildOf(player),
    ));

    // Visual sprite layer.
    commands.spawn((
        Sprite {
            image: player_texture,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        ChildOf(player),
    ));
}

pub fn spawn_platforms(commands: &mut Commands) {
    let platforms = [
        PlatformSpec {
            position: Vec2::new(0.0, -140.0),
            size: PLATFORM_SIZE,
            color: Color::srgb(0.2, 0.2, 0.25),
        },
        PlatformSpec {
            position: Vec2::new(140.0, -20.0),
            size: SMALL_PLATFORM_SIZE,
            color: Color::srgb(0.25, 0.25, 0.3),
        },
        PlatformSpec {
            position: Vec2::new(-160.0, 60.0),
            size: SMALL_PLATFORM_SIZE,
            color: Color::srgb(0.25, 0.25, 0.3),
        },
    ];
    for platform in platforms {
        commands.spawn((
            Ground,
            Platform {
                size: platform.size,
            },
            Sprite {
                color: platform.color,
                custom_size: Some(platform.size),
                ..default()
            },
            Transform::from_xyz(platform.position.x, platform.position.y, 0.0),
            RigidBody::Static,
            Collider::rectangle(platform.size.x, platform.size.y),
        ));
    }
}

pub fn spawn_coins(commands: &mut Commands, assets: &AssetServer) {
    let coin_texture = assets.load("textures/ferris.png");
    let base_left = -PLATFORM_SIZE.x * 0.5 + COIN_SIZE.x * 0.5 + 8.0;
    let coins = [
        Vec2::new(base_left, -100.0),
        Vec2::new(-160.0, 110.0),
        Vec2::new(140.0, 20.0),
    ];

    for position in coins {
        commands.spawn((
            Coin,
            Sprite {
                image: coin_texture.clone(),
                custom_size: Some(COIN_SIZE),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 1.0),
            RigidBody::Static,
            Collider::circle(COIN_RADIUS),
            Sensor,
        ));
    }
}

pub fn spawn_hud(commands: &mut Commands, fonts: &FontAssets) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        Text::new(""),
        TextFont {
            font: fonts.default.clone(),
            font_size: 42.0,
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        TextColor(HUD_TEXT_COLOR),
        HudText,
    ));
}

pub fn spawn_coin_counter(commands: &mut Commands, assets: &AssetServer) {
    let coin_texture = assets.load("textures/ferris.png");

    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                right: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("CoinCounter"),
        ))
        .id();

    for index in 0..COIN_SLOTS {
        commands.spawn((
            Node {
                width: Val::Px(COIN_ICON_SIZE.x),
                height: Val::Px(COIN_ICON_SIZE.y),
                ..default()
            },
            ImageNode {
                image: coin_texture.clone(),
                color: COIN_SLOT_EMPTY_COLOR,
                ..default()
            },
            CoinSlot { index },
            ChildOf(root),
        ));
    }
}
