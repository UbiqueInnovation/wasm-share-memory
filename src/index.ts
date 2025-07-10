import init, {
    get_heap_base,
    relocate as rust_relocate,
} from '../pkg/rust_module';
import wasmUrl from '../pkg/rust_module_bg.wasm';

function base64ToUint8Array(base64: string): Uint8Array {
    const binary = atob(base64); // decode base64 string
    const len = binary.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
}

const wasm = base64ToUint8Array(wasmUrl as unknown as string);
let initialized = false;

async function initialize() {
    if (!initialized) {
        await init({ module_or_path: wasm });
        initialized = true;
    }
}

export async function getHeapBase(module: Uint8Array) {
    await initialize();
    return get_heap_base(module);
}

export async function relocate(
    module: Uint8Array,
    offset: number,
    heapBase: number,
) {
    await initialize();
    return rust_relocate(module, offset, heapBase);
}

export async function combine(
    modules: Uint8Array[],
    additionalStack: number = 0,
) {
    const heapBases = await Promise.all(
        modules.map((module) => getHeapBase(module)),
    );

    const heapBase = heapBases.reduce(
        (acc, base) => acc + base + additionalStack,
        0,
    );

    const relocated = [];
    for (let i = 0; i < modules.length; i++) {
        const module = modules[i];
        const offset = heapBases
            .slice(0, i)
            .reduce((acc, base) => acc + base + additionalStack, 0);
        relocated.push(await relocate(module, offset, heapBase));
    }

    // floor + 1 rather than ceil, just in case to be safe
    const pagesNeeded = Math.floor(heapBase / 65536) + 1;

    return {
        modules: relocated,
        neededPages: pagesNeeded,
    };
}
