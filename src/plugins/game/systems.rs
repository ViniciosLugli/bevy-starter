use avian2d::prelude::*;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;

use crate::{
    components::{Coin, CoinSlot, Ground, HudText, Jump, Move, Platform, Player},
    resources::{CoinState, FpsDisplay},
    utils::format_hud_text,
};

use super::constants::{
    COIN_SLOT_EMPTY_COLOR, COIN_SLOT_FILLED_COLOR, COIN_SLOTS, JUMP_SPEED, PLAYER_SIZE,
    PLAYER_SPEED,
};

pub fn apply_player_input(
    mut players: Query<(&Transform, &mut LinearVelocity, &CollidingEntities), With<Player>>,
    move_action: Query<&ActionValue, With<Action<Move>>>,
    jump_action: Query<&ActionState, With<Action<Jump>>>,
    ground: Query<(&Transform, &Platform), With<Ground>>,
) {
    let Ok((player_transform, mut velocity, colliding)) = players.single_mut() else {
        return;
    };
    let Ok(move_value) = move_action.single() else {
        return;
    };
    let Ok(jump_state) = jump_action.single() else {
        return;
    };

    let movement = move_value.as_axis2d();
    velocity.x = movement.x * PLAYER_SPEED;

    let player_bottom = player_transform.translation.y - PLAYER_SIZE.y * 0.5;
    // Consider grounded only when the player is above the platform top.
    let grounded = colliding.iter().any(|entity| {
        let Ok((platform_transform, platform)) = ground.get(*entity) else {
            return false;
        };
        let platform_top = platform_transform.translation.y + platform.size.y * 0.5;
        player_bottom >= platform_top - 1.0
    });
    // Jump only when grounded and the action is fired.
    if grounded && *jump_state == ActionState::Fired {
        velocity.y = JUMP_SPEED;
    }
}

pub fn collect_coins(
    mut commands: Commands,
    mut coin_state: ResMut<CoinState>,
    players: Query<&CollidingEntities, With<Player>>,
    coins: Query<Entity, With<Coin>>,
) {
    let Ok(colliding) = players.single() else {
        return;
    };

    for entity in colliding.iter() {
        let Ok(coin_entity) = coins.get(*entity) else {
            continue;
        };
        commands.entity(coin_entity).despawn();
        coin_state.collected = (coin_state.collected + 1).min(COIN_SLOTS);
    }
}

pub fn update_hud(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut fps_display: ResMut<FpsDisplay>,
    move_action: Query<&ActionValue, With<Action<Move>>>,
    jump_action: Query<&ActionState, With<Action<Jump>>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let Ok(move_value) = move_action.single() else {
        return;
    };
    let Ok(jump_state) = jump_action.single() else {
        return;
    };
    let Ok(mut text) = text.single_mut() else {
        return;
    };

    fps_display.timer.tick(time.delta());
    if fps_display.timer.just_finished() || fps_display.value == 0.0 {
        fps_display.value = diagnostics
            .get_measurement(&FrameTimeDiagnosticsPlugin::FPS)
            .map(|value| value.value)
            .unwrap_or(0.0);
    }

    let movement = move_value.as_axis2d();
    text.0 = format_hud_text(fps_display.value, movement, *jump_state);
}

pub fn update_coin_counter(
    coin_state: Res<CoinState>,
    mut slots: Query<(&CoinSlot, &mut ImageNode)>,
) {
    if !coin_state.is_changed() {
        return;
    }

    for (slot, mut image) in slots.iter_mut() {
        image.color = if slot.index < coin_state.collected {
            COIN_SLOT_FILLED_COLOR
        } else {
            COIN_SLOT_EMPTY_COLOR
        };
    }
}
