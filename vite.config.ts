import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  build: {
    minify: false,
  },
  plugins: [wasmPack(['./markdown-render', './text-generation'])],
});
