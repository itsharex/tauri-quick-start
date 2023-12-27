<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import CustomWindow from './components/customWindow.vue'
</script>

<template>
  <div class="container">
    <CustomWindow />
    <h1>Tauri启动成功!</h1>
    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>该程序是由 Vue3 + Vite + Tauri构建的.</p>
    <ul>
      <li><a href='https://tauri.app/zh-cn/v1/guides/getting-started/setup/' target="_blank"><span>Tauri官网</span></a></li>
      <li><a href='https://tauri.app/zh-cn/v1/api/config#tauriconfig' target="_blank"><span>Tauri应用配置</span></a></li>
      <li><a href='https://www.electronjs.org/zh/' target="_blank"><span>GitHub源码地址</span></a></li>
    </ul>
    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
