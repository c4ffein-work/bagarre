/// Basic fighting game match example
///
/// This example demonstrates how to set up and run a basic fighting game match
/// using the Bagarre engine.

use bagarre::{
    Engine, InputState, Direction, GameResult, PlayerId,
};

fn main() {
    println!("=== Bagarre Fighting Game Engine - Basic Match Example ===\n");

    // Create engine with default configuration
    let mut engine = Engine::new();

    // You can also use custom configurations:
    // let config = EngineConfig::casual(); // Lenient inputs, quick matches
    // let config = EngineConfig::competitive(); // Strict inputs, standard rules
    // let config = EngineConfig::training(); // No time limit, high health

    // Initialize a standard 2-player match
    engine.init_match();

    println!("Match initialized!");
    println!("Player 1: Starting Health = {}",
             engine.get_player_entity(PlayerId::PLAYER_1).unwrap().health.current);
    println!("Player 2: Starting Health = {}\n",
             engine.get_player_entity(PlayerId::PLAYER_2).unwrap().health.current);

    // Simulate a fighting game match
    let mut frame = 0;
    let max_frames = 600; // 10 seconds at 60 FPS

    while frame < max_frames {
        frame += 1;

        // Create input for players (in a real game, this would come from controllers/keyboard)
        let p1_input = simulate_player1_input(frame);
        let p2_input = simulate_player2_input(frame);

        // Tick the engine forward one frame
        engine.tick(p1_input, p2_input);

        // Get current game state
        let state = engine.get_state();

        // Print status every 60 frames (1 second)
        if frame % 60 == 0 {
            let p1_health = engine.get_player_entity(PlayerId::PLAYER_1)
                .map(|e| e.health.current)
                .unwrap_or(0);
            let p2_health = engine.get_player_entity(PlayerId::PLAYER_2)
                .map(|e| e.health.current)
                .unwrap_or(0);

            println!("Frame {}: P1 Health: {} | P2 Health: {}",
                     state.frame, p1_health, p2_health);
        }

        // Check for match end
        if state.result != GameResult::InProgress {
            println!("\n=== MATCH END ===");
            println!("Result: {:?}", state.result);
            println!("Final Frame: {}", state.frame);
            break;
        }
    }

    if frame >= max_frames {
        println!("\n=== TIME OUT ===");
        let p1_health = engine.get_player_entity(PlayerId::PLAYER_1)
            .map(|e| e.health.current)
            .unwrap_or(0);
        let p2_health = engine.get_player_entity(PlayerId::PLAYER_2)
            .map(|e| e.health.current)
            .unwrap_or(0);

        if p1_health > p2_health {
            println!("Player 1 wins by timeout!");
        } else if p2_health > p1_health {
            println!("Player 2 wins by timeout!");
        } else {
            println!("Draw!");
        }
    }

    println!("\n=== Example Complete ===");
}

/// Simulate player 1 input (just for demonstration)
fn simulate_player1_input(frame: u64) -> InputState {
    let mut input = InputState::neutral();

    // Player 1 attacks every 120 frames (2 seconds)
    if frame % 120 == 30 {
        input.light = true;
    }

    // Occasionally do a quarter circle forward + special for a special move
    if frame % 180 == 60 {
        // In a real implementation, you'd set direction over multiple frames
        input.direction = Direction::Forward;
        input.special = true;
    }

    input
}

/// Simulate player 2 input (just for demonstration)
fn simulate_player2_input(frame: u64) -> InputState {
    let mut input = InputState::neutral();

    // Player 2 attacks every 100 frames
    if frame % 100 == 50 {
        input.medium = true;
    }

    // Player 2 tries to block occasionally
    if frame % 120 > 25 && frame % 120 < 35 {
        input.direction = Direction::Back;
    }

    input
}
