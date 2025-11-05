# 🥊 Bagarre Fighting Game Engine

A lightweight, zero-dependency fighting game engine written in Rust, inspired by the [Castagne](https://github.com/panthavma/castagne) engine architecture. Designed for competitive fighting games with frame-perfect gameplay and WASM browser support.

**"Bagarre"** (French) = fight, brawl, scuffle - a fitting name for a fighting game engine!

## ✨ Features

- **🚫 Zero Dependencies** - Core engine has NO external dependencies for maximum portability
- **⚡ Phase-Based Execution** - Deterministic, rollback-ready game loop inspired by Castagne
- **🎮 State Machine System** - Frame-perfect character states with hitbox/hurtbox collision
- **🎯 Input Buffering** - Motion detection for special moves (QCF, DP, HCF, etc.)
- **🌐 WASM Support** - Run in the browser with minimal bindings
- **🧪 Fully Tested** - Comprehensive test suite (23 tests, all passing)
- **🎯 Competitive Ready** - Frame data, combo systems, and cancels built-in

## 🏗️ Architecture

Inspired by Castagne's modular, phase-based design:

```
┌─────────────────────────────────────────┐
│         GAME TICK (60 FPS)              │
├─────────────────────────────────────────┤
│ 1. INPUT PHASE                          │
│    └─ Process player inputs             │
│    └─ Detect motion inputs (QCF, DP)    │
├─────────────────────────────────────────┤
│ 2. ACTION PHASE                         │
│    └─ Update entity states              │
│    └─ Execute state actions             │
│    └─ State machine transitions         │
├─────────────────────────────────────────┤
│ 3. PHYSICS PHASE                        │
│    └─ Collision detection               │
│    └─ Hitbox vs hurtbox checks          │
├─────────────────────────────────────────┤
│ 4. REACTION PHASE                       │
│    └─ Resolve hits and blocks           │
│    └─ Apply damage and knockback        │
├─────────────────────────────────────────┤
│ 5. CLEANUP PHASE                        │
│    └─ Check win conditions              │
│    └─ Update facing directions          │
└─────────────────────────────────────────┘
```

## 🚀 Quick Start

### As a Library

```rust
use bagarre::{Engine, InputState, Direction, GameResult};

fn main() {
    let mut engine = Engine::new();
    engine.init_match();

    loop {
        // Get player inputs (from keyboard, controller, etc.)
        let p1_input = InputState::neutral();
        let p2_input = InputState::neutral();

        // Tick the engine (advances one frame)
        engine.tick(p1_input, p2_input);

        // Get game state
        let state = engine.get_state();
        println!("Frame {}: P1 HP: {}, P2 HP: {}",
                 state.frame, state.p1_health, state.p2_health);

        // Check win condition
        if state.result != GameResult::InProgress {
            println!("Game over! Result: {:?}", state.result);
            break;
        }
    }
}
```

### Run Tests

```bash
cargo test
```

All 23 tests should pass:
- ✅ Core types (Vec2, Rect, Facing)
- ✅ Hitbox/hurtbox collision
- ✅ Input buffering and motion detection
- ✅ State machine transitions
- ✅ Entity physics and health
- ✅ Full game simulation

## 🌐 WASM Browser Build

### 🎮 Live Demo

**[Try it live on GitHub Pages!](https://c4ffein-work.github.io/bagarre/)** _(Auto-deployed on every push to main)_

### Prerequisites

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown
```

### Build for WASM

```bash
# Use the build script (recommended)
./build-wasm.sh

# Or build manually:
cargo build --target wasm32-unknown-unknown --release
# Copy to browser demo:
cp target/wasm32-unknown-unknown/release/bagarre.wasm examples/browser/
```

### Run Browser Demo Locally

```bash
# Build WASM first
./build-wasm.sh

# Serve the demo (requires a local server)
cd examples/browser
python3 -m http.server 8000
# Open http://localhost:8000
```

**Controls:**
- **Player 1:** WASD (move) + J/K/L (Light/Medium/Heavy) + U (Special)
- **Player 2:** Arrow Keys (move) + 1/2/3 (Light/Medium/Heavy) + 0 (Special)

The browser demo uses direct WASM loading without wasm-bindgen for true zero-dependency builds!

## 🎮 Core Systems

### 1. State Machine

Characters are driven by states with frame data:

```rust
use bagarre::state::{State, StateId, StateType, StateAction, FrameData};
use bagarre::hitbox::AttackData;

// Create a light attack state
let light_attack = State::new(StateId::LightAttack, StateType::Attack, 18)
    .with_cancel()  // Can cancel to other moves
    .add_frame_data(FrameData::new(5, StateAction::Hitbox {
        x: 15000,
        y: 10000,
        width: 12000,
        height: 8000,
        attack: AttackData::new(50)
            .with_stun(8, 6)
            .with_knockback(400, 0),
    }));
```

### 2. Input System

Supports motion inputs for special moves:

```rust
use bagarre::input::{InputBuffer, InputState, Direction};
use bagarre::types::Facing;

let mut buffer = InputBuffer::new(Facing::Right);

// Simulate quarter circle forward (Hadoken)
buffer.push(InputState { direction: Direction::Down, ..InputState::neutral() });
buffer.push(InputState { direction: Direction::DownForward, ..InputState::neutral() });
buffer.push(InputState { direction: Direction::Forward, ..InputState::neutral() });

if buffer.detect_qcf() {
    println!("Quarter circle forward detected!");
}
```

### 3. Collision System

AABB hitbox/hurtbox detection:

```rust
use bagarre::hitbox::{CollisionSystem, CollisionBox, AttackData};
use bagarre::types::{Rect, EntityId};

let mut system = CollisionSystem::new();

let hitbox = CollisionBox::hitbox(
    EntityId(0),
    Rect::new(10, 10, 20, 20),
    AttackData::new(100),
);

let hurtbox = CollisionBox::hurtbox(
    EntityId(1),
    Rect::new(15, 15, 20, 20),
);

system.add_hitbox(hitbox);
system.add_hurtbox(hurtbox);

let collisions = system.check_collisions();
// Handle collision results...
```

## 📊 Technical Details

### Coordinate System

- **Units**: Fixed-point integers (divide by 1000 for display)
- **Y-axis**: 0 = ground level, negative = airborne
- **X-axis**: 0 = center, negative = left, positive = right

### Frame Data

All timing is frame-based (60 FPS standard):
- **Startup**: Frames before hitbox becomes active
- **Active**: Frames hitbox is active
- **Recovery**: Frames after hitbox deactivates
- **Frame Advantage**: Difference in recovery between hit/block

### Input Buffer

- **Buffer Size**: 30 frames (0.5 seconds at 60 FPS)
- **Motion Detection Window**: 15 frames (0.25 seconds)
- **Button Press Window**: 3 frames

### Supported Motions

- **236** (QCF): Quarter Circle Forward - ↓↘→
- **214** (QCB): Quarter Circle Back - ↓↙←
- **623** (DP): Dragon Punch - →↓↘
- **41236** (HCF): Half Circle Forward
- **63214** (HCB): Half Circle Back

## 🧩 Module Structure

```
bagarre/
├── src/
│   ├── lib.rs           # Main library entry
│   ├── types.rs         # Core types (Vec2, Rect, Facing, etc.)
│   ├── hitbox.rs        # Hitbox/hurtbox system
│   ├── input.rs         # Input handling and motion detection
│   ├── state.rs         # State machine system
│   ├── entity.rs        # Fighter entity system
│   ├── engine.rs        # Main game engine
│   └── wasm.rs          # WASM interface (target-specific)
├── examples/
│   └── browser/         # Browser demo with HTML/JS
├── castagne-inspiration/ # Cloned Castagne for reference
└── Cargo.toml
```

## 🎯 Design Philosophy

### Inspired by Castagne

This engine takes inspiration from Castagne's excellent architecture:

1. **Phase-Based Execution**: Deterministic, predictable game loop
2. **Modular Design**: Systems are independent and composable
3. **State Machines**: Clean character behavior organization
4. **Frame Perfect**: Everything is frame-based for competitive play
5. **Rollback Ready**: Deterministic execution enables rollback netcode

### Key Differences from Castagne

- **Language**: Rust instead of GDScript/Godot
- **Dependencies**: Zero deps vs Godot framework
- **Scope**: Core engine only vs full game editor
- **Scripting**: Code-based vs .casp script language
- **Target**: Library/WASM vs Godot integration

## 🔬 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_collision_detection

# Run with coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

## 🛠️ Building from Source

```bash
# Clone the repository
git clone <your-repo-url>
cd bagarre

# Build
cargo build --release

# Run tests
cargo test

# Build for WASM
cargo build --target wasm32-unknown-unknown --release

# Build documentation
cargo doc --open
```

## 📝 Examples

### Custom Character State

```rust
use bagarre::state::State;

// Create a custom special move
let fireball = State::new(
    StateId::Custom(1),
    StateType::Attack,
    30  // 30 frames duration
).add_frame_data(FrameData::new(8, StateAction::Hitbox {
    x: 20000,
    y: 10000,
    width: 15000,
    height: 8000,
    attack: AttackData::new(80)
        .unblockable()
        .with_knockback(1000, -200),
}));
```

### Input Handling

```rust
// Check for special move input
if input_buffer.detect_qcf() && input_buffer.button_just_pressed(Button::Light) {
    state_machine.transition(StateId::SpecialMove);
}
```

## 🤝 Contributing

Contributions are welcome! This is an educational project inspired by Castagne.

## 📚 Resources

- [Castagne Engine](https://github.com/panthavma/castagne) - The inspiration for this project
- [Fighting Game Glossary](https://glossary.infil.net/) - Fighting game terminology
- [Frame Data](https://en.wikipedia.org/wiki/Frame_data_(video_gaming)) - Understanding frame data

## 📜 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- **Castagne Engine** by Panthavma - For the excellent architecture and design patterns
- The fighting game community - For decades of frame data research and competitive play

---

**Made with 🥊 and 🦀 Rust**
