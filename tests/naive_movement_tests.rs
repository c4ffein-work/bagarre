//! Super Naive Movement and Attack Tests
//!
//! Simple tests for basic player movement, jumping, and attacking.

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
fn test_player1_movement_and_jump() {
    println!("\n=== Player 1: Movement + Jump Test ===");

    let mut engine = Engine::new();
    engine.init_match();

    // Get initial position
    let p1_start = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;

    println!("P1 initial position: ({}, {})", p1_start.x, p1_start.y);

    // Test forward movement
    println!("Testing forward movement...");
    for _ in 0..30 {
        engine.tick(
            input_with_direction(Direction::Forward),
            InputState::neutral(),
        );
    }

    let p1_after_forward = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    println!(
        "P1 after moving forward: ({}, {})",
        p1_after_forward.x, p1_after_forward.y
    );
    assert!(
        p1_after_forward.x > p1_start.x,
        "P1 should have moved forward (x increased)"
    );

    // Return to neutral
    for _ in 0..20 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    // Test another direction - move in a different direction
    println!("Testing another direction (down-forward)...");
    for _ in 0..30 {
        engine.tick(
            input_with_direction(Direction::DownForward),
            InputState::neutral(),
        );
    }

    let p1_after_diagonal = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    println!(
        "P1 after diagonal movement: ({}, {})",
        p1_after_diagonal.x, p1_after_diagonal.y
    );
    // Diagonal forward should move more forward
    assert!(
        p1_after_diagonal.x > p1_after_forward.x,
        "P1 should have moved more forward with diagonal input"
    );

    // Return to neutral
    for _ in 0..10 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    // Test jump - input the jump command
    println!("Testing jump...");
    let y_before_jump = p1_after_diagonal.y;
    println!("P1 Y before jump: {}", y_before_jump);

    // Press up to jump
    engine.tick(input_with_direction(Direction::Up), InputState::neutral());

    // Check Y during the jump (should be in the air)
    for i in 0..8 {
        engine.tick(InputState::neutral(), InputState::neutral());

        // Check Y position during jump
        if i == 3 {
            let p1_during_jump = engine
                .get_player_entity(PlayerId::PLAYER_1)
                .unwrap()
                .physics
                .position;
            println!("P1 Y during jump (frame {}): {}", i, p1_during_jump.y);

            // Y should be negative (in the air) during jump
            // Ground is at Y=0, negative Y means airborne
            assert!(
                p1_during_jump.y < y_before_jump,
                "P1 should be in the air during jump (Y should be negative/lower than ground)"
            );
        }
    }

    let p1_after_jump = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    println!("P1 Y after jump sequence: {}", p1_after_jump.y);

    println!("✓ Player 1 movement and jump test passed!");
}

#[test]
fn test_player2_movement_and_jump() {
    println!("\n=== Player 2: Movement + Jump Test ===");

    let mut engine = Engine::new();
    engine.init_match();

    // Get initial position
    let p2_start = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;

    println!("P2 initial position: ({}, {})", p2_start.x, p2_start.y);

    // Test forward movement (for P2, this is moving left/back since they face left)
    println!("Testing forward movement...");
    for _ in 0..30 {
        engine.tick(
            InputState::neutral(),
            input_with_direction(Direction::Forward),
        );
    }

    let p2_after_forward = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;
    println!(
        "P2 after moving forward: ({}, {})",
        p2_after_forward.x, p2_after_forward.y
    );
    assert!(
        p2_after_forward.x < p2_start.x,
        "P2 should have moved forward (x decreased, toward P1)"
    );

    // Return to neutral
    for _ in 0..20 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    // Test another direction - move in a different direction
    println!("Testing another direction (down-forward)...");
    for _ in 0..30 {
        engine.tick(
            InputState::neutral(),
            input_with_direction(Direction::DownForward),
        );
    }

    let p2_after_diagonal = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;
    println!(
        "P2 after diagonal movement: ({}, {})",
        p2_after_diagonal.x, p2_after_diagonal.y
    );
    // Diagonal forward should move more forward (toward P1, so x decreases)
    assert!(
        p2_after_diagonal.x < p2_after_forward.x,
        "P2 should have moved more forward with diagonal input"
    );

    // Return to neutral
    for _ in 0..10 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    // Test jump - input the jump command
    println!("Testing jump...");
    let y_before_jump = p2_after_diagonal.y;
    println!("P2 Y before jump: {}", y_before_jump);

    // Press up to jump
    engine.tick(InputState::neutral(), input_with_direction(Direction::Up));

    // Check Y during the jump (should be in the air)
    for i in 0..8 {
        engine.tick(InputState::neutral(), InputState::neutral());

        // Check Y position during jump
        if i == 3 {
            let p2_during_jump = engine
                .get_player_entity(PlayerId::PLAYER_2)
                .unwrap()
                .physics
                .position;
            println!("P2 Y during jump (frame {}): {}", i, p2_during_jump.y);

            // Y should be negative (in the air) during jump
            // Ground is at Y=0, negative Y means airborne
            assert!(
                p2_during_jump.y < y_before_jump,
                "P2 should be in the air during jump (Y should be negative/lower than ground)"
            );
        }
    }

    let p2_after_jump = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;
    println!("P2 Y after jump sequence: {}", p2_after_jump.y);

    println!("✓ Player 2 movement and jump test passed!");
}

#[test]
fn test_player1_approaches_and_attacks_player2() {
    println!("\n=== Player 1 Approaches and Attacks Player 2 ===");

    let mut engine = Engine::new();
    engine.init_match();

    let initial_p2_health = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .health
        .current;

    println!("Initial P2 health: {}", initial_p2_health);

    // Both players move toward each other to close the gap
    println!("Players moving toward each other...");

    // P1 moves forward significantly (walk speed: 300 units/frame)
    // Need to close ~100000 unit gap to attack range (~30000)
    for _ in 0..200 {
        engine.tick(
            input_with_direction(Direction::Forward),
            InputState::neutral(),
        );
    }

    // P2 also moves forward to close the gap
    for _ in 0..80 {
        engine.tick(
            InputState::neutral(),
            input_with_direction(Direction::Forward),
        );
    }

    let p1_pos = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    let p2_pos = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;
    let distance = (p2_pos.x - p1_pos.x).abs();

    println!("P1 position: ({}, {})", p1_pos.x, p1_pos.y);
    println!("P2 position: ({}, {})", p2_pos.x, p2_pos.y);
    println!("Distance: {}", distance);

    // Now P1 does a light attack
    println!("P1 performing light attack...");
    let mut attack_input = InputState::neutral();
    attack_input.light = true;
    engine.tick(attack_input, InputState::neutral());

    // Wait for attack to complete and hit
    for _ in 0..20 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    let final_p2_health = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .health
        .current;

    println!("Final P2 health: {}", final_p2_health);
    println!("Damage dealt: {}", initial_p2_health - final_p2_health);

    assert!(
        final_p2_health < initial_p2_health,
        "P2 health should have decreased after P1's attack"
    );

    println!("✓ Player 1 successfully attacked Player 2!");
}

#[test]
fn test_player2_approaches_and_attacks_player1() {
    println!("\n=== Player 2 Approaches and Attacks Player 1 ===");

    let mut engine = Engine::new();
    engine.init_match();

    let initial_p1_health = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .health
        .current;

    println!("Initial P1 health: {}", initial_p1_health);

    // Both players move toward each other to close the gap
    println!("Players moving toward each other...");

    // P1 moves forward (walk speed: 300 units/frame)
    // Need to close ~100000 unit gap to attack range (~30000)
    for _ in 0..200 {
        engine.tick(
            input_with_direction(Direction::Forward),
            InputState::neutral(),
        );
    }

    // P2 also moves forward significantly
    for _ in 0..80 {
        engine.tick(
            InputState::neutral(),
            input_with_direction(Direction::Forward),
        );
    }

    let p1_pos = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .physics
        .position;
    let p2_pos = engine
        .get_player_entity(PlayerId::PLAYER_2)
        .unwrap()
        .physics
        .position;
    let distance = (p2_pos.x - p1_pos.x).abs();

    println!("P1 position: ({}, {})", p1_pos.x, p1_pos.y);
    println!("P2 position: ({}, {})", p2_pos.x, p2_pos.y);
    println!("Distance: {}", distance);

    // Now P2 does a light attack
    println!("P2 performing light attack...");
    let mut attack_input = InputState::neutral();
    attack_input.light = true;
    engine.tick(InputState::neutral(), attack_input);

    // Wait for attack to complete and hit
    for _ in 0..20 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    let final_p1_health = engine
        .get_player_entity(PlayerId::PLAYER_1)
        .unwrap()
        .health
        .current;

    println!("Final P1 health: {}", final_p1_health);
    println!("Damage dealt: {}", initial_p1_health - final_p1_health);

    assert!(
        final_p1_health < initial_p1_health,
        "P1 health should have decreased after P2's attack"
    );

    println!("✓ Player 2 successfully attacked Player 1!");
}
