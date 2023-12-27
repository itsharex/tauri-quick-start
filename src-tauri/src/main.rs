// 这个文件是Tauri的主入口配置文件，在这里可以实现Tauri系统级功能，例如系统托盘等
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入 窗口菜单 参考文档 https://tauri.app/zh-cn/v1/guides/features/menu
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// 前端代码调用 Rust命令 示例：https://tauri.app/zh-cn/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! 前面的文字是你输入的,这段话是通过Vue调用Rust生成的", name)
}

// 导入 系统拖托盘,示例：https://tauri.app/zh-cn/v1/guides/features/system-tray#configuring-a-system-tray-context-menu
use tauri::{Manager, SystemTray};

// 导入 系统托盘上下文
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "退出应用");

    let show = CustomMenuItem::new("show".to_string(), "显示应用");

    // 创建系统托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show);

    // 将托盘菜单添加到SystemTray实例中：
    let tray = SystemTray::new().with_menu(tray_menu);

    // 构建tauri
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // get a handle to the clicked menu item
                // note that `tray_handle` can be called anywhere,
                // just get an `AppHandle` instance with `app.handle()` on the setup hook
                // and move it to another function or thread
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {
                // 系统托盘图标点击事件
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
