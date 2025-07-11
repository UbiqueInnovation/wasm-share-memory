import { combine } from './public/index.mjs';

const valueElem = document.getElementById('value');
const doubledElem = document.getElementById('doubled');

const libcommon = new Uint8Array(
    await (await fetch('public/common.wasm')).arrayBuffer(),
);
const liba = new Uint8Array(await (await fetch('public/a.wasm')).arrayBuffer());
const libb = new Uint8Array(await (await fetch('public/b.wasm')).arrayBuffer());

// Layout:
// common | lib a | lib b | shared heap

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
valueElem.innerText = value.toString();

ib.exports.double(object);

const doubled = ia.exports.get_value(object);
doubledElem.innerText = doubled.toString();
