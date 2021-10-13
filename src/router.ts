import { VueRouter } from "vue-router";
/*
 * @Author: Lin Bowen
 * @Date: 2021-10-13 16:28:11
 * @LastEditTime: 2021-10-13 17:39:23
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\src\router.ts
 */
import { createRouter, createWebHashHistory } from "vue-router";
import MD5 from "./pages/md5.vue";
import Maze from "./pages/maze.vue";
const routes = [
  { path: "/md5", component: MD5 },
  { path: "/maze", component: Maze },
];

export default createRouter({
  history: createWebHashHistory(),
  routes,
});
