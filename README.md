<div align="center">

# WASM Share Memory

**Relocatable WebAssembly Modules with Shared Memory Layouts**

</div>

Combine multiple WebAssembly (WASM) modules into a single linear memory space — enabling efficient shared-memory communication without linking at compile time.

## ✨ Features

- 🧠 **Relocates multiple standalone WASM modules into a shared memory space**
- 🧩 **Preserves individual module logic and memory layout**
- 🧾 **Modifies globals and memory segments for compatibility**

## 📦 Installation

```bash
npm install @ubique-innovation/wasm-share-memory
```

## 🛠️ Usage

The WASM modules must be compiled with the following `RUSTFLAGS` and compiler flags:

```bash
# import-memory: allows the WASM module to import memory from the host environment
# relocation-model=pic: enables position-independent code for relocatable modules
export RUSTFLAGS="-C link-arg=--import-memory -Crelocation-model=pic"

# Currently, the nightly toolchain is required, because the std, alloc, and core libraries also need to be compiled with `relocation-model=pic`, otherwise they will generate absolute addresses.
cargo +nightly build -Z build-std="core,std,alloc,panic_abort" --target wasm32-unknown-unknown --release
```

```bash
# Install the package
pnpm install @ubique-innovation/wasm-share-memory
```

````ts
import * as fs from 'fs';
import { combine } from '@ubique-innovation/wasm-share-memory';

// Load the WASM modules as Uint8Arrays
const libcommon = fs.readFileSync('./output/common.wasm');
const liba = fs.readFileSync('./output/a.wasm');
const libb = fs.readFileSync('./output/b.wasm');

// Layout: common | liba | libb | shared heap
const { modules, neededPages } = await combine([libcommon, liba, libb]);

// Create shared memory
const memory = new WebAssembly.Memory({
    initial: neededPages,
});

// Instantiate modules with the shared memory
const { instance: icommon } = await WebAssembly.instantiate(modules[0], {
    env: { memory },
});
const { instance: ia } = await WebAssembly.instantiate(modules[1], {
    env: { memory },
});
const { instance: ib } = await WebAssembly.instantiate(modules[2], {
    env: { memory },
});

// Example usage of shared memory:
//
// ```rust
// // common.rs
// struct Object { value: i64 }
//
// pub fn create_object(value: i64) -> *mut Object;
//
// // liba.rs
// pub fn get_value(object: &Object) -> i64;
//
// // libb.rs
// pub fn double(object: *mut Object);
// ```

// Instantiate an 'Object' using the common library.
const object = icommon.exports.create_object(1337n);

// Use the 'get_value' function from liba to read the value.
const value = ia.exports.get_value(object);
console.log(value); // 1337n

// Use the 'double' function from libb to modify the value.
ib.exports.double(object);

// Read the modified value back from liba.
// This will return the doubled value.
const doubled = ia.exports.get_value(object);
console.log(doubled); // 2674n
````

## 🧬 How It Works

- Uses a Rust+WASM backend (via [`walrus`](https://github.com/rustwasm/walrus)) to:
    - Inspect and rewrite WASM modules.
    - Patch `__wasm_init_memory` to initialize the shifted memory.
    - Patch all global variables with values between `__stack_pointer` and `__heap_base`.
    - Relocate globals and data segments to prevent overlap.
    - Patch `__heap_base`, `__stack_pointer`, and `__memory_base`.
- JavaScript/TypeScript wrapper loads and prepares modules in the browser or Node.js.

For more internal details, check the [Rust source code](rust-lib/src/lib.rs).

## 🧪 Functions

### `combine(modules: Uint8Array[], additionalStack?: number): Promise<{ modules, neededPages }>`

Relocates modules sequentially and returns modified binaries and required memory pages for the shared memory layout.

### `getHeapBase(module: Uint8Array): Promise<number>`

Extracts the `__heap_base` global.

### `relocate(module: Uint8Array, offset: number, newHeapBase: number): Promise<Uint8Array>`

Relocates a single module to a new memory offset and adjusts `__heap_base`.

## 📂 Project Structure

- `rust-lib/`: WASM backend implementing the relocation logic. Packaged using `wasm-pack`.
- `src/index.ts`: TS wrapper logic for loading and interacting with the backend
- `example/`: Example project demonstrating usage in a NodeJS environment.
- `example/www/`: Example web project demonstrating usage in a browser environment.

## 🚀 Building

### Package

```bash
# Install dependencies
pnpm install

# Build the rust library (WASM) and the package
pnpm build

# Build only the Rust library
pnpm run build:wasm

# Build only the package (depends on the WASM library)
pnpm run build:package
```

### Example

```bash
cd example

# Install dependencies
pnpm install

# Build the example rust libraries
./build.sh

# Run the example
pnpm start
```

### Example (Web)

```bash
cd example/www

# Install dependencies
pnpm install

# Build the example rust libraries and copy them
./build.sh

# Start the web server
python3 -m http.server 8000
```

## 📄 License

[MIT](LICENSE)
