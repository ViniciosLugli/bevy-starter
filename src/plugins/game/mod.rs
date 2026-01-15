use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    components::PlayerInput,
    plugins::fonts::FontAssets,
    resources::{CoinState, FpsDisplay},
};

mod constants;
mod spawn;
mod systems;

use spawn::*;
use systems::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>()
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<CoinState>()
        .init_resource::<FpsDisplay>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_player_input,
                collect_coins,
                update_hud,
                update_coin_counter,
            ),
        );
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, fonts: Res<FontAssets>) {
    spawn_player(&mut commands, &assets);
    spawn_platforms(&mut commands);
    spawn_coins(&mut commands, &assets);
    spawn_hud(&mut commands, &fonts);
    spawn_coin_counter(&mut commands, &assets);
}
