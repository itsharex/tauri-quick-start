<!-- 自定义顶部窗口样式，需要在 tauri.confi.json中关闭 默认Windows窗口样式 windows.decorations = false， 如果不想要自定义窗口样式，设置  windows.decorations = true , 将当前组件 隐藏-->
<!-- 官网示例：https://tauri.app/zh-cn/v1/guides/features/window-customization#html -->
<!-- 建议使用自定义窗口菜单，因为原生窗口顶部菜单事件处理很麻烦，没有 appWindow 操作简单 -->
<script setup>
// Tauri 窗口api: https://tauri.app/zh-cn/v1/api/js/
import { appWindow } from '@tauri-apps/api/window';
function minimize() {
  appWindow.minimize();
}
function toggleMaximize() {
  appWindow.toggleMaximize()
}
function close() {
  // 不关闭应用而是隐藏 appWindow.close()
  appWindow.hide()
}
</script>
<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button" id="titlebar-minimize" @click="minimize">
      <img src="@/assets/images/window/minimize.png" alt="minimize" />
    </div>
    <div class="titlebar-button" id="titlebar-maximize" @click="toggleMaximize">
      <img src="@/assets/images/window/maximize-1.png" alt="toggleMaximize" />
    </div>
    <div class="titlebar-button" id="titlebar-close" @click="close">
      <img src="@/assets/images/window/close.png" alt="close" />
    </div>
  </div>
</template>
<style  scoped>
.titlebar {
  height: 30px;
  background-color: rgba(255, 255, 255, .5);
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}

.titlebar-button:hover {
  background: #767777;
}
</style>
