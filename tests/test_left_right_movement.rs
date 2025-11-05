//! Tests for Left and Right Movement
//!
//! Verifies that both players can move in both directions (left and right)
//! using Forward (toward opponent) and Back (away from opponent) directions.

use bagarre::{Direction, Engine, InputState, PlayerId};

/// Helper to create an input with a specific direction
fn input_with_direction(dir: Direction) -> InputState {
    InputState {
        direction: dir,
        light: false,
        medium: false,
        heavy: false,
        special: false,
    }
}

#[test]
fn test_player1_forward_movement_goes_right() {
    println!("\n=== P1 Forward Movement (Should Go Right) ===");

    let mut engine = Engine::new();
    engine.init_match();

    let p1_start = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;

    println!("P1 starts at x={}", p1_start.x);

    // P1 faces right, so Forward should move right (x increases)
    for _ in 0..60 {
        engine.tick(
            input_with_direction(Direction::Forward),
            InputState::neutral(),
        );
    }

    let p1_after = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;

    println!("P1 after Forward: x={}", p1_after.x);
    println!("Δx = {}", p1_after.x - p1_start.x);

    assert!(
        p1_after.x > p1_start.x,
        "P1 Forward should move right (x increases)"
    );
}

#[test]
fn test_player1_back_movement_goes_left() {
    println!("\n=== P1 Back Movement (Should Go Left) ===");

    let mut engine = Engine::new();
    engine.init_match();

    let p1_start = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;

    println!("P1 starts at x={}", p1_start.x);

    // P1 faces right, so Back should move left (x decreases)
    for _ in 0..60 {
        engine.tick(input_with_direction(Direction::Back), InputState::neutral());
    }

    let p1_after = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;

    println!("P1 after Back: x={}", p1_after.x);
    println!("Δx = {}", p1_after.x - p1_start.x);

    assert!(
        p1_after.x < p1_start.x,
        "P1 Back should move left (x decreases)"
    );
}

#[test]
fn test_player2_forward_movement_goes_left() {
    println!("\n=== P2 Forward Movement (Should Go Left) ===");

    let mut engine = Engine::new();
    engine.init_match();

    let p2_start = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    println!("P2 starts at x={}", p2_start.x);

    // P2 faces left, so Forward should move left (x decreases)
    for _ in 0..60 {
        engine.tick(
            InputState::neutral(),
            input_with_direction(Direction::Forward),
        );
    }

    let p2_after = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    println!("P2 after Forward: x={}", p2_after.x);
    println!("Δx = {}", p2_after.x - p2_start.x);

    assert!(
        p2_after.x < p2_start.x,
        "P2 Forward should move left (x decreases)"
    );
}

#[test]
fn test_player2_back_movement_goes_right() {
    println!("\n=== P2 Back Movement (Should Go Right) ===");

    let mut engine = Engine::new();
    engine.init_match();

    let p2_start = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    println!("P2 starts at x={}", p2_start.x);

    // P2 faces left, so Back should move right (x increases)
    for _ in 0..60 {
        engine.tick(InputState::neutral(), input_with_direction(Direction::Back));
    }

    let p2_after = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    println!("P2 after Back: x={}", p2_after.x);
    println!("Δx = {}", p2_after.x - p2_start.x);

    assert!(
        p2_after.x > p2_start.x,
        "P2 Back should move right (x increases)"
    );
}

#[test]
fn test_both_players_can_retreat() {
    println!("\n=== Both Players Retreat Simultaneously ===");

    let mut engine = Engine::new();
    engine.init_match();

    let p1_start = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    let p2_start = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    let initial_distance = (p2_start.x - p1_start.x).abs();
    println!("Initial distance: {}", initial_distance);

    // Both players back away from each other
    for _ in 0..60 {
        engine.tick(
            input_with_direction(Direction::Back),
            input_with_direction(Direction::Back),
        );
    }

    let p1_after = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    let p2_after = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    let final_distance = (p2_after.x - p1_after.x).abs();
    println!("Final distance: {}", final_distance);
    println!(
        "Distance increased by: {}",
        final_distance - initial_distance
    );

    assert!(
        final_distance > initial_distance,
        "Both players backing away should increase distance"
    );
    assert!(p1_after.x < p1_start.x, "P1 should have moved left");
    assert!(p2_after.x > p2_start.x, "P2 should have moved right");
}
