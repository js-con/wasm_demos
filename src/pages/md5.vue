<!--
 * @Author: Lin Bowen
 * @Date: 2021-10-13 16:31:40
 * @LastEditTime: 2021-10-13 16:41:19
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\src\pages\md5.vue
-->
<template>
  <div>
    <input type="file" ref="input" />
    <button @click="jsStart">JS-sparkMD5计算</button>
    <button @click="RustStart">Rust-md5计算</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "@vue/reactivity";
import { onMounted } from "@vue/runtime-dom";
import SparkMD5 from "spark-md5";
import init, { RustMD5 } from "wasm";

const input = ref<{ files: Array<Blob> } | null>(null);

let wasm = null;
onMounted(async () => {
  wasm = await init();
});

const jsStart = () => {
  const file = input.value?.files[0];
  if (!file) return;
  const reader = new FileReader();
  reader.readAsBinaryString(file);
  reader.onload = (e) => {
    console.time("JS计算md5");
    const result = SparkMD5.hash(e.target?.result as string);
    console.timeEnd("JS计算md5");
    console.log(result);
  };
};
const RustStart = () => {
  const file = input.value?.files[0];
  if (!file) return;
  const reader = new FileReader();
  reader.readAsBinaryString(file);
  reader.onload = (e) => {
    console.time("rust计算md5");
    const result = RustMD5(e.target?.result as string);
    console.timeEnd("rust计算md5");
    console.log(result);
  };
  wasm.RustMD5;
};
</script>


<style lang='scss'>

</style>