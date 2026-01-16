//! Font assets.

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<FontAssets>()
        .init_resource::<FontAssets>();
}

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub(crate) struct FontAssets {
    pub default: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let default: Handle<Font> = assets.load("fonts/PixelSmall.ttf");

        Self { default }
    }
}
