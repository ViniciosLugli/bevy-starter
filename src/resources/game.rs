use bevy::prelude::*;

#[derive(Resource)]
pub struct CoinState {
    pub collected: usize,
}

impl Default for CoinState {
    fn default() -> Self {
        Self { collected: 0 }
    }
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
