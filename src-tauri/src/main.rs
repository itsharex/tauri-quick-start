// 这个文件是Tauri的主入口配置文件，在这里可以实现Tauri系统级功能，例如系统托盘等
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入 窗口菜单 参考文档 https://tauri.app/zh-cn/v1/guides/features/menu
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// 前端代码调用 Rust命令 示例：https://tauri.app/zh-cn/v1/guides/features/command
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! 前面的文字是你输入的,这段话是通过Vue调用Rust生成的",
        name
    )
}

// 导入 系统拖托盘,示例：https://tauri.app/zh-cn/v1/guides/features/system-tray#configuring-a-system-tray-context-menu
use tauri::{Manager, SystemTray};

// 导入 系统托盘上下文
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

// 导入窗口菜单，如果使用自定义窗口样式，不会生效
use tauri::{Menu, Submenu};

fn main() {
    // 创建窗口顶部菜单 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    // 窗口菜单事件和 系统托盘事件写法一样
    let windowHome = CustomMenuItem::new("home".to_string(), "首页");
    let windowSubmenu = Submenu::new(
        "快捷跳转页面",
        Menu::new().add_item(windowHome)
    );
    let windowMenu = Menu::new().add_submenu(windowSubmenu);

    // 创建系统托盘菜单 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "退出应用");

    let show = CustomMenuItem::new("show".to_string(), "显示应用");

    // 创建系统托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // 将托盘菜单添加到SystemTray实例中：
    let tray = SystemTray::new().with_menu(tray_menu);

    // 构建tauri
    tauri::Builder::default()
        .menu(windowMenu)
        .on_menu_event(|event| match event.menu_item_id() {
            "home" => {
                println!("点击了首页!");
            }
            _ => (),
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // get a handle to the clicked menu item
                // note that `tray_handle` can be called anywhere,
                // just get an `AppHandle` instance with `app.handle()` on the setup hook
                // and move it to another function or thread
                // let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        println!("点击了退出应用!");
                        app.exit(0);
                    }
                    "show" => {
                        println!("点击了显示应用!");
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
