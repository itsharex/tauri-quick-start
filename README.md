# 一个最基础的快速启动 Tauri程序，实现了最基础常用功能

Tauri依赖 `Rust`环境，开发 Tauri需要安装对应的环境，请参考[预先准备](https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites)


基础是 Tauri + Vue3，可用于 桌面应用开发

根目录`index.html`是入口文件，和Vue项目中的index.html理解一样

`src` 文件夹 是 Vue 项目源文件, 也可以是其他任意前端源文件`最好是用Vue，因为实现了自定义窗口样式，请查看src/components/customWindow.vue组件,使用其他前端实现它 `(对源文件进行开发，最终打包静态文件移动到dist，打包编译桌面应用)
`src-tauri` 是 Tauri 项目配置文件夹 ，图标信息等
`dist` 存放的是 已经打包需要部署的静态页面文件，Tauri打包会将这个文件夹的所有文件编译为桌面应用

启动打包命令 参考: `package.json` 文件

 启动 vue 项目
```cmd
npm run dev
```
编译打包 vue项目 
```cmd
npm run build 
```
使用桌面端启动 vue项目
```cmd
npm run tauri dev 
```
打包vue 为桌面应用 exe
```cmd
npm run tauri build 
```

# 示例1：
将 Vue打包好的 dist文件夹复制到 当前目录下覆盖dist文件夹
然后执行 npm run tauri build 命令打包为桌面应用