import { defineConfig } from 'tsup';

export default defineConfig({
    entry: ['src/index.ts'],
    format: ['esm'],
    sourcemap: true,
    dts: true,
    splitting: false,
    clean: true,
    bundle: true,
    loader: {
        '.wasm': 'base64',
    },
});
