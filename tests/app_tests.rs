use avian2d::prelude::*;
use bevy::prelude::*;
use starter::{Coin, CoinSlot, CoinState, Player, collect_coins, update_coin_counter};

fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins); // Headless schedule for fast tests.
    app
}

#[test]
fn collect_coins_despawns_and_counts() {
    let mut app = setup_app();
    // Prepare minimal state so collection logic has data to mutate.
    app.insert_resource(CoinState::default());

    // Arrange a colliding player to exercise the collection path.
    let coin_entity = app.world_mut().spawn(Coin).id();
    let mut colliding = CollidingEntities::default();
    colliding.insert(coin_entity); // Simulate collision so collect_coins runs.
    app.world_mut().spawn((Player, colliding));

    // Drive the schedule once to apply the system effects.
    app.add_systems(Update, collect_coins);
    app.update();

    // Validate that collection removed the coin and updated the counter.
    assert!(app.world().get_entity(coin_entity).is_err());
    assert_eq!(app.world().resource::<CoinState>().collected, 1);
}

#[test]
fn update_coin_counter_sets_slot_colors() {
    let mut app = setup_app();
    // Use a known count to make the expected colors deterministic.
    app.insert_resource(CoinState::default());
    app.world_mut().resource_mut::<CoinState>().collected = 2; // Trigger change detection with known count.

    // Seed UI slots so the system can update their colors.
    for index in 0..4 {
        app.world_mut().spawn((
            CoinSlot { index },
            ImageNode {
                color: Color::srgb(0.35, 0.35, 0.35),
                ..default()
            },
        ));
    }

    // Run once to apply the slot color changes.
    app.add_systems(Update, update_coin_counter);
    app.update();

    // Verify colors match collected count.
    let mut query = app.world_mut().query::<(&CoinSlot, &ImageNode)>(); // Borrow from world_mut to satisfy query lifetime.
    for (slot, image) in query.iter(app.world()) {
        let expected = if slot.index < 2 {
            Color::WHITE
        } else {
            Color::srgb(0.35, 0.35, 0.35)
        };
        assert_eq!(image.color, expected);
    }
}
