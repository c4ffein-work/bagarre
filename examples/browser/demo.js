// Bagarre Fighting Game Engine - Browser Demo
// This demo uses the actual WASM-compiled engine

let wasmModule = null;
let gameRunning = false;
let paused = false;
let frameInterval = null;
let useWasm = true; // Set to false for simulation mode

// Input state
const p1Input = {
    up: false, down: false, left: false, right: false,
    light: false, medium: false, heavy: false, special: false
};

const p2Input = {
    up: false, down: false, left: false, right: false,
    light: false, medium: false, heavy: false, special: false
};

// State names
const STATE_NAMES = [
    'Idle', 'Walk', 'Crouch', 'Jump',
    'Light', 'Medium', 'Heavy', 'Special',
    'Hit', 'Block', 'Down'
];

// Initialize the game
async function initGame() {
    try {
        console.log('🥊 Initializing Bagarre...');

        // Try to load WASM module
        if (useWasm) {
            wasmModule = new BagarreWasm();
            await wasmModule.load('bagarre.wasm');
            console.log('✅ WASM module loaded successfully');
            document.getElementById('status').textContent = 'Ready! Use keyboard to fight!';
        } else {
            console.log('ℹ️ Running in simulation mode');
            document.getElementById('status').textContent = 'Simulation Mode - Ready!';
        }

        // Start the game loop
        startGame();
    } catch (err) {
        console.error('Failed to load WASM:', err);
        console.log('⚠️ Falling back to simulation mode');

        // Fallback to simulation mode
        useWasm = false;
        wasmModule = null;
        document.getElementById('status').textContent =
            'Running in simulation mode (WASM not available)';
        startGame();
    }
}

// Encode input to bitfield format for WASM
function encodeInput(input) {
    let direction = 5; // Neutral

    if (input.down && input.left) direction = 1;
    else if (input.down && input.right) direction = 3;
    else if (input.down) direction = 2;
    else if (input.up && input.left) direction = 7;
    else if (input.up && input.right) direction = 9;
    else if (input.up) direction = 8;
    else if (input.left) direction = 4;
    else if (input.right) direction = 6;

    let encoded = direction;
    if (input.light) encoded |= 0x10;
    if (input.medium) encoded |= 0x20;
    if (input.heavy) encoded |= 0x40;
    if (input.special) encoded |= 0x80;

    return encoded;
}

// Start the game
function startGame() {
    gameRunning = true;

    // If WASM is loaded, initialize it
    if (wasmModule && wasmModule.init) {
        wasmModule.init();
    }

    // Run at 60 FPS
    frameInterval = setInterval(gameLoop, 1000 / 60);

    // Set up keyboard listeners
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('keyup', handleKeyUp);
}

// Main game loop
function gameLoop() {
    if (paused) return;

    // Encode inputs
    const p1Encoded = encodeInput(p1Input);
    const p2Encoded = encodeInput(p2Input);

    // Update game
    if (useWasm && wasmModule) {
        wasmModule.tick(p1Encoded, p2Encoded);
        updateDisplay();
    } else {
        // Simulation mode for demo
        updateDisplaySimulation();
    }

    // Render
    render();
}

// Update display from WASM state
function updateDisplay() {
    if (!wasmModule) return;

    // Get state from WASM
    const frame = wasmModule.get_frame();
    const p1X = wasmModule.get_p1_x();
    const p1Y = wasmModule.get_p1_y();
    const p1Health = wasmModule.get_p1_health();
    const p1State = wasmModule.get_p1_state();
    const p1Facing = wasmModule.get_p1_facing();

    const p2X = wasmModule.get_p2_x();
    const p2Y = wasmModule.get_p2_y();
    const p2Health = wasmModule.get_p2_health();
    const p2State = wasmModule.get_p2_state();
    const p2Facing = wasmModule.get_p2_facing();

    const result = wasmModule.get_result();

    // Update UI
    document.getElementById('frame').textContent = frame;

    document.getElementById('p1-pos').textContent = `${p1X}, ${p1Y}`;
    document.getElementById('p1-hp').textContent = p1Health;
    document.getElementById('p1-state').textContent = STATE_NAMES[p1State] || 'Unknown';
    document.getElementById('p1-facing').textContent = p1Facing > 0 ? '→' : '←';
    document.getElementById('p1-health').style.width = (p1Health / 10) + '%';

    document.getElementById('p2-pos').textContent = `${p2X}, ${p2Y}`;
    document.getElementById('p2-hp').textContent = p2Health;
    document.getElementById('p2-state').textContent = STATE_NAMES[p2State] || 'Unknown';
    document.getElementById('p2-facing').textContent = p2Facing > 0 ? '→' : '←';
    document.getElementById('p2-health').style.width = (p2Health / 10) + '%';

    // Check win condition
    if (result === 1) {
        document.getElementById('status').innerHTML = '<span class="winner">PLAYER 1 WINS!</span>';
        paused = true;
    } else if (result === 2) {
        document.getElementById('status').innerHTML = '<span class="winner">PLAYER 2 WINS!</span>';
        paused = true;
    } else if (result === 3) {
        document.getElementById('status').innerHTML = '<span class="winner">DRAW!</span>';
        paused = true;
    }
}

// Simulation mode (for demo without WASM)
let simFrame = 0;
let simP1Health = 1000;
let simP2Health = 1000;
let simP1X = -50000;
let simP2X = 50000;

function updateDisplaySimulation() {
    simFrame++;
    document.getElementById('frame').textContent = simFrame;

    // Simple simulation
    if (p1Input.right) simP1X += 300;
    if (p1Input.left) simP1X -= 300;
    if (p2Input.right) simP2X += 300;
    if (p2Input.left) simP2X -= 300;

    document.getElementById('p1-pos').textContent = `${simP1X}, 0`;
    document.getElementById('p1-hp').textContent = simP1Health;
    document.getElementById('p1-health').style.width = (simP1Health / 10) + '%';
    document.getElementById('p1-state').textContent =
        p1Input.light || p1Input.medium || p1Input.heavy ? 'Attack' : 'Idle';

    document.getElementById('p2-pos').textContent = `${simP2X}, 0`;
    document.getElementById('p2-hp').textContent = simP2Health;
    document.getElementById('p2-health').style.width = (simP2Health / 10) + '%';
    document.getElementById('p2-state').textContent =
        p2Input.light || p2Input.medium || p2Input.heavy ? 'Attack' : 'Idle';
}

// Render game to canvas
function render() {
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');

    // Clear
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw ground
    ctx.strokeStyle = '#00ff00';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(0, canvas.height - 50);
    ctx.lineTo(canvas.width, canvas.height - 50);
    ctx.stroke();

    // Get positions
    let p1x, p2x, p1y, p2y;
    const groundY = canvas.height - 50;

    if (useWasm && wasmModule) {
        // Get real positions from WASM
        p1x = wasmModule.get_p1_x() / 1000 + canvas.width / 2 - 20;
        p2x = wasmModule.get_p2_x() / 1000 + canvas.width / 2 - 20;
        p1y = groundY - (wasmModule.get_p1_y() / 1000);
        p2y = groundY - (wasmModule.get_p2_y() / 1000);
    } else {
        // Use simulation positions
        p1x = simP1X / 1000 + canvas.width / 2 - 100;
        p2x = simP2X / 1000 + canvas.width / 2 - 100;
        p1y = groundY;
        p2y = groundY;
    }

    // Draw player 1
    ctx.fillStyle = '#00ff00';
    ctx.fillRect(p1x, p1y - 80, 40, 80);
    ctx.fillStyle = '#00aa00';
    ctx.fillRect(p1x + 10, p1y - 90, 20, 20); // Head

    // Draw player 2
    ctx.fillStyle = '#ff0000';
    ctx.fillRect(p2x, p2y - 80, 40, 80);
    ctx.fillStyle = '#aa0000';
    ctx.fillRect(p2x + 10, p2y - 90, 20, 20); // Head

    // Draw attack indicators
    if (p1Input.light || p1Input.medium || p1Input.heavy) {
        ctx.strokeStyle = '#ffff00';
        ctx.lineWidth = 3;
        ctx.strokeRect(p1x + 40, p1y - 60, 30, 20);
    }

    if (p2Input.light || p2Input.medium || p2Input.heavy) {
        ctx.strokeStyle = '#ffff00';
        ctx.lineWidth = 3;
        ctx.strokeRect(p2x - 30, p2y - 60, 30, 20);
    }
}

// Keyboard input
function handleKeyDown(e) {
    // Player 1 (WASD + JKLU)
    if (e.key === 'w' || e.key === 'W') p1Input.up = true;
    if (e.key === 's' || e.key === 'S') p1Input.down = true;
    if (e.key === 'a' || e.key === 'A') p1Input.left = true;
    if (e.key === 'd' || e.key === 'D') p1Input.right = true;
    if (e.key === 'j' || e.key === 'J') p1Input.light = true;
    if (e.key === 'k' || e.key === 'K') p1Input.medium = true;
    if (e.key === 'l' || e.key === 'L') p1Input.heavy = true;
    if (e.key === 'u' || e.key === 'U') p1Input.special = true;

    // Player 2 (Arrows + 1230)
    if (e.key === 'ArrowUp') p2Input.up = true;
    if (e.key === 'ArrowDown') p2Input.down = true;
    if (e.key === 'ArrowLeft') p2Input.left = true;
    if (e.key === 'ArrowRight') p2Input.right = true;
    if (e.key === '1') p2Input.light = true;
    if (e.key === '2') p2Input.medium = true;
    if (e.key === '3') p2Input.heavy = true;
    if (e.key === '0') p2Input.special = true;

    e.preventDefault();
}

function handleKeyUp(e) {
    // Player 1
    if (e.key === 'w' || e.key === 'W') p1Input.up = false;
    if (e.key === 's' || e.key === 'S') p1Input.down = false;
    if (e.key === 'a' || e.key === 'A') p1Input.left = false;
    if (e.key === 'd' || e.key === 'D') p1Input.right = false;
    if (e.key === 'j' || e.key === 'J') p1Input.light = false;
    if (e.key === 'k' || e.key === 'K') p1Input.medium = false;
    if (e.key === 'l' || e.key === 'L') p1Input.heavy = false;
    if (e.key === 'u' || e.key === 'U') p1Input.special = false;

    // Player 2
    if (e.key === 'ArrowUp') p2Input.up = false;
    if (e.key === 'ArrowDown') p2Input.down = false;
    if (e.key === 'ArrowLeft') p2Input.left = false;
    if (e.key === 'ArrowRight') p2Input.right = false;
    if (e.key === '1') p2Input.light = false;
    if (e.key === '2') p2Input.medium = false;
    if (e.key === '3') p2Input.heavy = false;
    if (e.key === '0') p2Input.special = false;

    e.preventDefault();
}

// UI controls
function restartGame() {
    if (useWasm && wasmModule) {
        wasmModule.init();
    }
    simFrame = 0;
    simP1Health = 1000;
    simP2Health = 1000;
    simP1X = -50000;
    simP2X = 50000;
    paused = false;
    document.getElementById('status').textContent = 'Fight!';
}

function togglePause() {
    paused = !paused;
    document.getElementById('status').textContent = paused ? 'Paused' : 'Fight!';
}

// Start when page loads
window.addEventListener('load', initGame);
