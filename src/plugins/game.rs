use avian2d::prelude::*;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::FontSmoothing,
};
use bevy_enhanced_input::prelude::*;

use crate::{plugins::fonts::FontAssets, utils::format_hud_text};

const PLAYER_SPEED: f32 = 240.0;
const JUMP_SPEED: f32 = 520.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(64.0);
const PLATFORM_SIZE: Vec2 = Vec2::new(520.0, 28.0);
const SMALL_PLATFORM_SIZE: Vec2 = Vec2::new(180.0, 20.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct PlayerInput;

#[derive(Component)]
struct HudText;

#[derive(Component, Clone, Copy)]
struct Platform {
    size: Vec2,
}

#[derive(Clone, Copy)]
struct PlatformSpec {
    position: Vec2,
    size: Vec2,
    color: Color,
}

#[derive(Resource)]
struct FpsDisplay {
    timer: Timer,
    value: f64,
}

impl Default for FpsDisplay {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            value: 0.0,
        }
    }
}

#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

pub(crate) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>()
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<FpsDisplay>()
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_player_input, update_hud));
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, fonts: Res<FontAssets>) {
    spawn_player(&mut commands, &assets);
    spawn_platforms(&mut commands);
    spawn_hud(&mut commands, &fonts);
}

fn spawn_player(commands: &mut Commands, assets: &AssetServer) {
    let player_texture = assets.load("textures/bevy.png");
    let player = commands
        .spawn((
            Player,
            PlayerInput,
            Transform::from_xyz(0.0, 0.0, 0.0),
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

fn spawn_platforms(commands: &mut Commands) {
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

fn spawn_hud(commands: &mut Commands, fonts: &FontAssets) {
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
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        HudText,
    ));
}

fn apply_player_input(
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

fn update_hud(
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
