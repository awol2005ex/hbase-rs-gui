use commands::{hbase_config::*,hbase_api::*};

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
            //获取hbase命名空间列表
            get_hbase_namespace_list_command,
            //获取hbase命名空间列表(增加统计信息)
            get_hbase_namespace_metrics_list_command,
            //获取hbase命名空间下表列表
            get_hbase_table_list_command,
            //获取hbase命名空间下表列表(增加统计信息)
            get_hbase_table_metrics_list_command,
            //分页查询数据
            get_hbase_table_data_list_command,
            //查询总行数
            get_hbase_table_data_count_command,
            //创建表
            create_table_command,
            //创建表空间
            create_namespace_command,
            //删除表空间
            delete_namespace_command,
            //删除表
            delete_table_command,
            //启用表
            enable_table_command,
            //禁用表
            disable_table_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
