import * as fs from 'fs';
import { combine } from 'wasm-share-memory';

const libcommon = fs.readFileSync('./output/common.wasm');
const liba = fs.readFileSync('./output/a.wasm');
const libb = fs.readFileSync('./output/b.wasm');

// Layout:
// common | lib a | lib b | heap

const { modules, neededPages } = await combine([libcommon, liba, libb]);

// Instantiate

const memory = new WebAssembly.Memory({
    initial: neededPages,
});

const { instance: icommon } = await WebAssembly.instantiate(modules[0], {
    env: { memory },
});

const { instance: ia } = await WebAssembly.instantiate(modules[1], {
    env: { memory },
});

const { instance: ib } = await WebAssembly.instantiate(modules[2], {
    env: { memory },
});

// Test

const object = icommon.exports.create_object(1337n);

const value = ia.exports.get_value(object);
console.log(value); // should print 1337n

ib.exports.double(object);

const doubled = ia.exports.get_value(object);
console.log(doubled); // should print 1674n
