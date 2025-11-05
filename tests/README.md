# End-to-End (E2E) Tests for Bagarre

This directory contains comprehensive end-to-end tests that simulate complete fighting game matches and realistic gameplay scenarios.

## Overview

The E2E tests go beyond unit testing to verify that the entire engine works correctly in real-world fighting game situations. These tests simulate multiple frames of gameplay, player interactions, and complete fight scenarios.

### Test Files

1. **`e2e_tests.rs`** (13 tests) - Core mechanics and system integration tests
2. **`e2e_full_fights.rs`** (9 tests) - **Complete full fight simulations between characters**

## Test Coverage (22 Total E2E Tests)

### Full Fight Simulations (`e2e_full_fights.rs` - 9 tests) ⭐ NEW!

These tests simulate **complete fights from start to finish** with realistic character interactions:

- **test_full_fight_aggressive_p1_vs_passive_p2**: P1 relentlessly attacks passive P2 until KO
- **test_full_fight_evenly_matched**: Both players trade attacks evenly in balanced combat
- **test_full_fight_rushdown_vs_zoner**: Rushdown character vs defensive zoning strategy
- **test_full_fight_comeback_scenario**: P2 dominates early, P1 makes dramatic comeback
- **test_full_fight_counter_hit_heavy**: Testing counter-hit mechanics and timing
- **test_full_fight_perfect_victory_p1**: P1 wins without taking any damage
- **test_full_fight_intense_exchange**: Rapid back-and-forth combat with constant pressure
- **test_full_fight_defensive_masterclass**: Defensive blocking strategy vs aggression
- **test_full_fight_timeout_scenario**: Cautious play leading to potential timeout

### Core Mechanics Tests (`e2e_tests.rs` - 13 tests)

## Detailed Test Coverage

### Complete Fight Simulations
- **test_complete_fight_p1_wins_by_ko**: Simulates a full match where Player 1 defeats Player 2 through repeated attacks
- **test_complete_fight_long_match**: Simulates 600 frames (10 seconds) of varied combat
- **test_complete_fight_simultaneous_ko**: Tests both players attacking simultaneously

### Combat Mechanics
- **test_complete_fight_combo_sequence**: Tests a full combo chain (Light → Medium → Heavy)
- **test_complete_fight_with_blocking**: Verifies blocking reduces or prevents damage
- **test_complete_fight_both_players_attacking**: Tests simultaneous attacks from both players
- **test_complete_fight_aggressive_vs_defensive**: Simulates different playstyle strategies

### Special Moves & Input Detection
- **test_complete_fight_special_move_qcf**: Tests Quarter Circle Forward motion (↓↘→)
- **test_complete_fight_dragon_punch**: Tests Dragon Punch motion (→↓↘)
- **test_complete_fight_frame_perfect_inputs**: Verifies precise frame timing for combos

### State Management
- **test_complete_fight_hitstun_and_blockstun**: Verifies hitstun and blockstun frame counts
- **test_complete_fight_knockback_and_momentum**: Tests knockback physics and momentum decay
- **test_complete_fight_movement_and_spacing**: Verifies player movement and facing direction

### Frame-Perfect Gameplay
- **test_complete_fight_frame_perfect_inputs**: Tests precise frame-by-frame input sequences

## Running the Tests

Run all E2E tests:
```bash
cargo test --test e2e_tests --test e2e_full_fights
```

Run only core mechanics tests:
```bash
cargo test --test e2e_tests
```

Run only full fight simulations:
```bash
cargo test --test e2e_full_fights
```

Run a specific E2E test:
```bash
cargo test --test e2e_tests test_complete_fight_combo_sequence
cargo test --test e2e_full_fights test_full_fight_comeback_scenario
```

Run with detailed output:
```bash
cargo test --test e2e_full_fights -- --nocapture
```

## Test Results

Current status: **All 22 E2E tests passing** ✅
- Core mechanics tests: **13 passing** ✅
- Full fight simulations: **9 passing** ✅

These tests complement the 35 unit/integration tests in `src/lib.rs` for a total of **57 comprehensive tests**.

## What the E2E Tests Verify

1. **Engine Stability**: The engine runs for hundreds of frames without crashing
2. **Combat Flow**: Attacks, blocking, hitstun, and combos work correctly
3. **Physics**: Movement, knockback, and momentum behave as expected
4. **Input System**: Motion detection and button presses are recognized
5. **State Machines**: Character states transition correctly
6. **Win Conditions**: The engine correctly determines match outcomes
7. **Realistic Scenarios**: The engine handles varied playstyles and strategies

## Test Design Philosophy

The E2E tests are designed to:
- **Simulate realistic gameplay**: Tests reflect actual fighting game scenarios
- **Handle spacing**: Tests account for attack range and positioning
- **Be deterministic**: Same inputs always produce same results
- **Test edge cases**: Simultaneous attacks, perfect inputs, long matches
- **Verify stability**: Engine runs without crashes or undefined behavior

## Adding New E2E Tests

When adding new E2E tests:

1. **Move players close together** if testing attacks:
   ```rust
   for _ in 0..60 {
       engine.tick(input_with_direction(Direction::Forward), InputState::neutral());
   }
   ```

2. **Use helper functions** for input creation:
   - `input_with_direction(dir)` - Create directional input
   - `input_with_button(button)` - Create button press input

3. **Test complete scenarios**, not just single frames

4. **Add descriptive output** with `println!` for debugging

5. **Account for spacing** - attacks may miss if players are too far apart

## Test Maintenance

These tests are maintained alongside the engine code and should be updated when:
- New combat mechanics are added
- Existing mechanics change behavior
- New character states are introduced
- Input detection logic is modified

## Performance

All E2E tests complete in under **0.5 seconds** total, making them suitable for rapid iteration during development.
