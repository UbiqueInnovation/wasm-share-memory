import * as fs from 'fs';
import { getHeapBase, relocate } from 'wasm-share-memory';

const runtime = fs.readFileSync('./output/runtime.wasm');
const libcommon = fs.readFileSync('./output/common.wasm');
const liba = fs.readFileSync('./output/a.wasm');
const libb = fs.readFileSync('./output/b.wasm');

// Layout:
// common | lib a | lib b | runtime | heap

const baseCommon = await getHeapBase(libcommon);
const baseA = await getHeapBase(liba);
const baseB = await getHeapBase(libb);
const baseRuntime = await getHeapBase(runtime);

console.log(`Heap Base Common: ${baseCommon}`);
console.log(`Heap Base A: ${baseA}`);
console.log(`Heap Base B: ${baseB}`);
console.log(`Heap Base Runtime: ${baseRuntime}`);

const absoluteHeapBase = baseCommon + baseA + baseB + baseRuntime;

const patchedCommon = await relocate(libcommon, 0, absoluteHeapBase);
const patchedLibA = await relocate(liba, baseCommon, absoluteHeapBase);
const patchedLibB = await relocate(libb, baseCommon + baseA, absoluteHeapBase);
const patchedRuntime = await relocate(
    runtime,
    baseCommon + baseA + baseB,
    absoluteHeapBase,
);

const baseCommonPatched = await getHeapBase(patchedCommon);
const baseAPatched = await getHeapBase(patchedLibA);
const baseBPatched = await getHeapBase(patchedLibB);
const baseRuntimePatched = await getHeapBase(patchedRuntime);

console.log(`Patched Heap Base Common: ${baseCommonPatched}`);
console.log(`Patched Heap Base A: ${baseAPatched}`);
console.log(`Patched Heap Base B: ${baseBPatched}`);
console.log(`Patched Heap Base Runtime: ${baseRuntimePatched}`);

// Instantiate

const pagesNeeded = Math.floor(absoluteHeapBase / 65536) + 1;
console.log(`Memory pages needed: ${pagesNeeded}`);

const memory = new WebAssembly.Memory({
    initial: pagesNeeded,
});

const { instance: irun } = await WebAssembly.instantiate(patchedRuntime, {
    env: { memory },
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

const value = ia.exports.get_value(object);
console.log(value); // should print 1337n

ib.exports.double(object);

const doubled = ia.exports.get_value(object);
console.log(doubled); // should print 1674n
