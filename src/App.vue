<!--
 * @Author: Lin Bowen
 * @Date: 2021-10-12 21:02:37
 * @LastEditTime: 2021-10-13 18:38:46
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\src\App.vue
-->
<template>
  <header>
    <ul>
      <li>
        <router-link to="/md5">md5</router-link>
      </li>
      <li>
        <router-link to="/maze">Maze</router-link>
      </li>
    </ul>
  </header>
  <main>
    <keep-alive>
      <router-view></router-view>
    </keep-alive>
  </main>
</template>

<script setup lang="ts"></script>

<style>
body{
  margin: 0;
  padding: 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
ul{
  margin: 0;
}
ul li {
  list-style: none;
  margin-bottom: 12px;
}
header {
  width: 100%;
  margin-bottom: 50px;
  background-color: #999;
}
main {
  position: relative;
  width: 100%;
}
</style>
