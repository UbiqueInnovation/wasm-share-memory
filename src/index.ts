import init, {
    get_heap_base,
    relocate as rust_relocate,
} from '../pkg/rust_module';
import wasmUrl from '../pkg/rust_module_bg.wasm';

let initialized = false;

async function initialize() {
    if (!initialized) {
        await init(wasmUrl);
        initialized = true;
    }
}

export async function getHeapBase(module: Uint8Array) {
    await initialize();
    return get_heap_base(module);
}

export async function relocate(module: Uint8Array, offset: number) {
    await initialize();
    return rust_relocate(module, offset);
}
