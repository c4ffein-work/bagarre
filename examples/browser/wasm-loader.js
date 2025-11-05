// WASM Loader for Bagarre Fighting Game Engine
// Loads the raw WASM module and exposes functions

class BagarreWasm {
    constructor() {
        this.instance = null;
        this.memory = null;
    }

    async load(wasmPath = 'bagarre.wasm') {
        try {
            // Fetch and instantiate WASM module
            const response = await fetch(wasmPath);
            const buffer = await response.arrayBuffer();

            const wasmModule = await WebAssembly.instantiate(buffer, {
                env: {
                    // Add any imports the WASM module needs here
                    // Our module has no external dependencies
                }
            });

            this.instance = wasmModule.instance;
            this.memory = this.instance.exports.memory;

            console.log('✅ WASM module loaded successfully');
            console.log('Exported functions:', Object.keys(this.instance.exports));

            return true;
        } catch (error) {
            console.error('Failed to load WASM:', error);
            throw error;
        }
    }

    // Initialize the engine
    init() {
        if (!this.instance) throw new Error('WASM not loaded');
        this.instance.exports.init();
    }

    // Update game by one frame
    tick(p1Input, p2Input) {
        if (!this.instance) throw new Error('WASM not loaded');
        this.instance.exports.tick(p1Input, p2Input);
    }

    // Getters for game state
    get_frame() {
        if (!this.instance) throw new Error('WASM not loaded');
        return Number(this.instance.exports.get_frame());
    }

    get_p1_x() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p1_x();
    }

    get_p1_y() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p1_y();
    }

    get_p1_health() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p1_health();
    }

    get_p1_state() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p1_state();
    }

    get_p1_facing() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p1_facing();
    }

    get_p2_x() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p2_x();
    }

    get_p2_y() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p2_y();
    }

    get_p2_health() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p2_health();
    }

    get_p2_state() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p2_state();
    }

    get_p2_facing() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_p2_facing();
    }

    get_result() {
        if (!this.instance) throw new Error('WASM not loaded');
        return this.instance.exports.get_result();
    }
}

// Export for use in demo.js
window.BagarreWasm = BagarreWasm;
