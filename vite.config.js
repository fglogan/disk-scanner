import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    tailwindcss()
  ],
  root: 'src',
  build: {
    outDir: '../dist',
  },
  server: {
    port: 3001,
    strictPort: true,
  },
})