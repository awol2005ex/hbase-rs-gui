use commands::{hbase_config::*};

mod commands;
mod db;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            //获取hbase配置列表
            get_hbase_config_list,
            //保存hbase设置
            save_hbase_config,
            //删除hbase设置
            delete_hbase_config,
            //获取hbase配置
            get_hbase_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
