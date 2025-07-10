import { getHeapBase, relocate } from './public/index.mjs';

const valueElem = document.getElementById('value');
const doubledElem = document.getElementById('doubled');

const runtime = new Uint8Array(
    await (await fetch('public/runtime.wasm')).arrayBuffer(),
);
const libcommon = new Uint8Array(
    await (await fetch('public/common.wasm')).arrayBuffer(),
);
const liba = new Uint8Array(await (await fetch('public/a.wasm')).arrayBuffer());
const libb = new Uint8Array(await (await fetch('public/b.wasm')).arrayBuffer());

// Layout:
// common | lib a | lib b | runtime | heap

const baseCommon = await getHeapBase(libcommon);
const baseA = await getHeapBase(liba);
const baseB = await getHeapBase(libb);

console.log(`Heap Base Common: ${baseCommon}`);
console.log(`Heap Base A: ${baseA}`);
console.log(`Heap Base B: ${baseB}`);

const absoluteHeapBase = baseCommon + baseA + baseB;

const patchedCommon = await relocate(libcommon, 0, absoluteHeapBase);
const patchedLibA = await relocate(liba, baseCommon, absoluteHeapBase);
const patchedLibB = await relocate(libb, baseCommon + baseA, absoluteHeapBase);

const baseCommonPatched = await getHeapBase(patchedCommon);
const baseAPatched = await getHeapBase(patchedLibA);
const baseBPatched = await getHeapBase(patchedLibB);

console.log(`Patched Heap Base Common: ${baseCommonPatched}`);
console.log(`Patched Heap Base A: ${baseAPatched}`);
console.log(`Patched Heap Base B: ${baseBPatched}`);

// Instantiate

const pagesNeeded = Math.floor(absoluteHeapBase / 65536) + 1;
console.log(`Memory pages needed: ${pagesNeeded}`);

const memory = new WebAssembly.Memory({
    initial: pagesNeeded,
});

const { instance: icommon } = await WebAssembly.instantiate(patchedCommon, {
    env: { memory },
});

const { instance: ia } = await WebAssembly.instantiate(patchedLibA, {
    env: { memory },
});

const { instance: ib } = await WebAssembly.instantiate(patchedLibB, {
    env: { memory },
});

// Test

const object = icommon.exports.create_object(1337n);
console.log('obj:', object);

const value = ia.exports.get_value(object);
valueElem.innerText = value.toString();

ib.exports.double(object);

const doubled = ia.exports.get_value(object);
doubledElem.innerText = doubled.toString();
