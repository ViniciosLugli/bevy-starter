use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CoinState {
    pub collected: usize,
}

#[derive(Resource)]
pub struct FpsDisplay {
    pub timer: Timer,
    pub value: f64,
}

impl Default for FpsDisplay {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            value: 0.0,
        }
    }
}
