/// Custom configuration example
///
/// This example demonstrates how to create custom game configurations
/// to tune physics, input handling, and game rules.

use bagarre::{
    Engine, InputState, PhysicsConfig, InputConfig, GameConfig, EngineConfig,
    GameResult,
};

fn main() {
    println!("=== Bagarre - Custom Configuration Example ===\n");

    // Example 1: Preset configurations
    println!("1. Using Preset Configurations:");
    demonstrate_presets();

    // Example 2: Custom physics
    println!("\n2. Custom Physics Configuration:");
    demonstrate_custom_physics();

    // Example 3: Custom input settings
    println!("\n3. Custom Input Configuration:");
    demonstrate_custom_input();

    // Example 4: Custom game rules
    println!("\n4. Custom Game Rules:");
    demonstrate_custom_rules();

    // Example 5: Building a complete custom config
    println!("\n5. Complete Custom Configuration:");
    demonstrate_complete_custom();

    println!("\n=== Example Complete ===");
}

fn demonstrate_presets() {
    // Casual mode: Lenient inputs, lower health, shorter rounds
    let casual = EngineConfig::casual();
    println!("  Casual Mode:");
    println!("    - Detection window: {} frames", casual.input.detection_window);
    println!("    - Starting health: {}", casual.game.starting_health);
    println!("    - Rounds to win: {}", casual.game.rounds_to_win);

    // Competitive mode: Strict inputs, standard rules
    let competitive = EngineConfig::competitive();
    println!("  Competitive Mode:");
    println!("    - Detection window: {} frames", competitive.input.detection_window);
    println!("    - Starting health: {}", competitive.game.starting_health);

    // Training mode: No time limit, very high health
    let training = EngineConfig::training();
    println!("  Training Mode:");
    println!("    - Time limit: {} (infinite)", training.game.time_limit_frames);
    println!("    - Starting health: {}", training.game.starting_health);
}

fn demonstrate_custom_physics() {
    // Create custom physics with high gravity and fast momentum decay
    let physics = PhysicsConfig {
        gravity: 160, // Double the default gravity
        ground_level: 18000,
        momentum_decay_percent: 70, // Faster decay (less slidey)
        knockback_threshold: -100,
    };

    println!("  Custom High Gravity, Fast Decay:");
    println!("    - Gravity: {}", physics.gravity);
    println!("    - Momentum decay: {}%", physics.momentum_decay_percent);
    println!("    - Effect: Faster falling, less momentum carry");

    // Can also use preset physics
    let floaty = PhysicsConfig::low_gravity();
    println!("  Preset Low Gravity:");
    println!("    - Gravity: {}", floaty.gravity);
    println!("    - Effect: Slower falling, more air time");
}

fn demonstrate_custom_input() {
    // Create custom input config with very lenient motion detection
    let input = InputConfig {
        buffer_size: 30,
        detection_window: 25, // Very large window for easier specials
    };

    println!("  Lenient Motion Detection:");
    println!("    - Detection window: {} frames", input.detection_window);
    println!("    - Effect: Easier to perform special moves");

    // Strict input for competitive play
    let strict = InputConfig::strict();
    println!("  Strict Motion Detection:");
    println!("    - Detection window: {} frames", strict.detection_window);
    println!("    - Effect: Requires precise input timing");
}

fn demonstrate_custom_rules() {
    // Create a quick match configuration
    let quick = GameConfig {
        starting_health: 500,
        time_limit_frames: 1800, // 30 seconds at 60 FPS
        rounds_to_win: 1,
    };

    println!("  Quick Match:");
    println!("    - Health: {}", quick.starting_health);
    println!("    - Time: {} frames ({} seconds)",
             quick.time_limit_frames,
             quick.time_limit_frames / 60);
    println!("    - Rounds: Best of {}", quick.rounds_to_win);

    // Create an extended match configuration
    let extended = GameConfig::extended_match();
    println!("  Extended Match:");
    println!("    - Health: {}", extended.starting_health);
    println!("    - Time: {} seconds", extended.time_limit_frames / 60);
    println!("    - Rounds: Best of {}", extended.rounds_to_win);
}

fn demonstrate_complete_custom() {
    // Build a complete custom configuration
    let custom_physics = PhysicsConfig::high_gravity();
    let custom_input = InputConfig::lenient();
    let custom_game = GameConfig {
        starting_health: 1500,
        time_limit_frames: 5400, // 90 seconds
        rounds_to_win: 2,
    };

    let config = EngineConfig::new(custom_physics, custom_input, custom_game);

    println!("  Custom 'Arcade' Configuration:");
    println!("    - High gravity with lenient inputs");
    println!("    - Health: {}", config.game.starting_health);
    println!("    - Time: {} seconds", config.game.time_limit_frames / 60);
    println!("    - Input window: {} frames", config.input.detection_window);

    // Note: In the current engine implementation, configs are for reference only.
    // A future version would allow passing config to Engine::new() to apply these settings.
    println!("\n  Note: Configuration system is ready for future integration with Engine.");
    println!("  Currently serves as a blueprint for custom game modes.");
}

#[allow(dead_code)]
fn run_match_with_config(_config: EngineConfig) {
    // This demonstrates how configs could be used in a future version
    println!("\nRunning match with custom config...");

    let mut engine = Engine::new();
    // Future: let mut engine = Engine::with_config(config);

    engine.init_match();

    // Simulate a few frames
    for _ in 0..60 {
        engine.tick(InputState::neutral(), InputState::neutral());
    }

    let state = engine.get_state();
    if state.result == GameResult::InProgress {
        println!("Match in progress after 1 second");
    }
}
