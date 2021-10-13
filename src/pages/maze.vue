<!--
 * @Author: Lin Bowen
 * @Date: 2021-10-13 16:45:31
 * @LastEditTime: 2021-10-13 18:20:41
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\src\pages\maze.vue
-->
<template>
  <canvas ref="canvas" width="800" height="800"></canvas>
</template>

<script setup>
import { Maze } from "../utils/maze";
import { ref } from "@vue/reactivity";
import { onMounted } from "@vue/runtime-dom";

const canvas = ref(null);
onMounted(() => {
  const column = 200,
    row = 200;
  const maze = new Maze(column, row, canvas.value);

  console.time("generate maze");
  maze.generate();
  console.timeEnd("generate maze");

  console.time("draw maze");
  maze.draw();
  console.timeEnd("draw maze");
});
</script>

<style lang="scss"></style>
