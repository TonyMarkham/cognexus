import init, { Renderer } from '../wasm/cognexus_renderer.js';

let initialized = false;

export async function createRenderer(canvas, width, height) {
    if (!initialized) {
        await init();
        initialized = true;
    }
    return await Renderer.new(canvas, width, height);
}
