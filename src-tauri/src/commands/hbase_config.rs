use serde::{Deserialize, Serialize};

use crate::db::db_init::DB_POOL;
use std::process::Command;
//hbase配置
#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct HbaseConfig {
    pub id: i64,
    pub name: String,
    pub hbase_config: String, //json配置
    pub hbase_env: String, //json  环境变量配置
    pub del_flag: i64, //0正常 1删除
}
//获取hbase配置列表
#[tauri::command]
pub async fn get_hbase_config_list() -> Result<Vec<HbaseConfig>, String> {
    crate::db::db_init::init_db()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(pool) = DB_POOL.get() {
        let hbase_config_list: Vec<HbaseConfig> =
            sqlx::query_as::<_, HbaseConfig>("select * from hbase_config where del_flag = 0")
                .fetch_all(pool)
                .await
                .unwrap_or(vec![]);
        return Ok(hbase_config_list);
    }
    Ok(vec![])
}

//保存hbase配置
#[tauri::command]
pub async fn save_hbase_config(hbase_config: HbaseConfig) -> Result<(), String> {
    crate::db::db_init::init_db()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(pool) = DB_POOL.get() {
        if hbase_config.id > 0 {
            sqlx::query(
                "update hbase_config set name = ?, hbase_config = ?, hbase_env = ? where id = ?",
            )
            .bind(hbase_config.name)
            .bind(hbase_config.hbase_config)
            .bind(hbase_config.hbase_env)
            .bind(hbase_config.id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            sqlx::query("insert into hbase_config (name, hbase_config, hbase_env,del_flag) values (?, ?, ? ,0)")
        .bind(hbase_config.name)
        .bind(hbase_config.hbase_config)
        .bind(hbase_config.hbase_env)
        .execute(pool).await.map_err(|e| e.to_string())?;
        }
    } else {
        return Err("Database connection pool is not initialized".to_owned());
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_hbase_config(id: i64) -> Result<(), String> {
    crate::db::db_init::init_db()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(pool) = DB_POOL.get() {
        sqlx::query("update hbase_config set del_flag=1 where id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        return Err("Database connection pool is not initialized".to_owned());
    }
    Ok(())
}


//获取单个hbase配置
pub async fn get_one_hbase_config(id: i64) -> Result<HbaseConfig, String> {
    if let Some(pool) = DB_POOL.get() {
        let hbase_config_list: Vec<HbaseConfig> =
            sqlx::query_as::<_, HbaseConfig>("select * from hbase_config where id=?")
                .bind(&id)
                .fetch_all(pool)
                .await
                .unwrap_or(vec![]);
        if hbase_config_list.len() > 0 {
            return Ok(hbase_config_list[0].clone());
        } else {
            return Err("no config found".to_owned());
        }
    } else {
        return Err("Database connection pool is not initialized".to_owned());
    }
}
//获取单个hbase配置
#[tauri::command]
pub async fn get_hbase_config(id: i64) -> Result<HbaseConfig, String> {
    crate::db::db_init::init_db()
        .await
        .map_err(|e| e.to_string())?;

    return get_one_hbase_config(id).await.map_err(|e| e.to_string());
}