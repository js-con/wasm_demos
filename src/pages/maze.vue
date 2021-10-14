<!--
 * @Author: Lin Bowen
 * @Date: 2021-10-13 16:45:31
 * @LastEditTime: 2021-10-15 02:59:38
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \wasm_demos\src\pages\maze.vue
-->
<template>
  <canvas ref="canvas" width="800" height="800"></canvas>
  <button @click="jsGen">JS生成迷宫</button>
  <button @click="rustGen">Rust生成迷宫</button>
</template>

<script setup>
import { Maze } from "../utils/maze";
import { ref } from "@vue/reactivity";
import { onMounted } from "@vue/runtime-dom";
import init, { RustMaze } from "wasm";

let wasm = null;
const canvas = ref(null);
onMounted(async () => {
  wasm = await init().catch(console.error);
});
const jsGen = () => {

  canvas.value.getContext('2d').clearRect(0,0,800,800)

  const maze = new Maze(100, 100, canvas.value);

  console.time("generate maze");
  maze.generate();
  console.timeEnd("generate maze");

  console.time("draw maze");
  maze.draw();
  console.timeEnd("draw maze");
};
const rustGen = () => {

  canvas.value.getContext('2d').clearRect(0,0,800,800)

  const maze = new RustMaze(100, 100, canvas.value);
  console.time("generate maze");
  maze.generate();
  console.timeEnd("generate maze");

  console.time("draw maze");
  maze.draw();
  console.timeEnd("draw maze");
};
</script>

<style lang="scss"></style>
