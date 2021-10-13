/*
 * @Author: Lin Bowen
 * @Date: 2021-10-13 09:57:54
 * @LastEditTime: 2021-10-13 16:39:30
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\vite.config.js
 */
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasmPack from 'vite-plugin-wasm-pack'
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),wasmPack('./wasm')]
})
