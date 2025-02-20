use std::collections::HashMap;

use j4rs::{ClasspathEntry, Instance, InvocationArg, JavaClass, JavaOpt, Jvm, JvmBuilder};
use serde::{Deserialize, Serialize};

pub struct HbaseOper {
    pub jvm: Jvm,
    pub hbase_tool: Instance,
    pub hbase_conf_map: std::collections::HashMap<String, String>,
    pub hbase_env_map: std::collections::HashMap<String, String>,
}
//获取单个hbase jvm 连接
pub fn get_hbase_oper(id: i64) -> Result<HbaseOper, String> {
    // 获取程序执行的当前目录
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    let jar_path = format!(
        "{}/hbase-oper-1.0-SNAPSHOT-all.jar",
        current_dir.to_str().unwrap_or_default().replace("\\", "/")
    );

    let entry = ClasspathEntry::new(&jar_path);
    //println!("{}", &jar_path);
    // Create a JVM
    let jvm = JvmBuilder::new()
        .java_opt(JavaOpt::new(
            "--add-exports=java.security.jgss/sun.security.krb5=ALL-UNNAMED",
        ))
        .classpath_entry(entry)
        .build()
        .map_err(|e| e.to_string())?;

    let hbase_config =
        futures::executor::block_on(crate::commands::hbase_config::get_hbase_config(id))?;

    let hbase_conf: serde_json::Value =
        serde_json::from_str(&hbase_config.hbase_config).map_err(|e| e.to_string())?;

    let hbase_conf_map = hbase_conf
        .as_object()
        .ok_or("hbase_conf parse error")?
        .iter()
        .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
        .collect::<std::collections::HashMap<String, String>>();

    let hbase_env: serde_json::Value =
        serde_json::from_str(&hbase_config.hbase_env).map_err(|e| e.to_string())?;

    let hbase_env_map = hbase_env
        .as_object()
        .ok_or("hbase_env parse error")?
        .iter()
        .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
        .collect::<std::collections::HashMap<String, String>>();

    let hbase_tool = jvm
        .create_instance("com.awol2005ex.hbase.HbaseTool", InvocationArg::empty())
        .map_err(|e| e.to_string())?;

    Ok(HbaseOper {
        jvm: jvm,
        hbase_tool: hbase_tool,
        hbase_conf_map: hbase_conf_map,
        hbase_env_map: hbase_env_map,
    })
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HbaseTableStatus {
    name: String,
    namespace: String,
    enabled: bool,
}
impl HbaseOper {
    pub fn get_hbase_namespace_list(&self) -> Result<Vec<String>, String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

        let namespaces_java_instance = self
            .jvm
            .invoke(
                &self.hbase_tool,
                "getNamespaces",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        let namespaces: Vec<String> = self
            .jvm
            .to_rust(namespaces_java_instance)
            .map_err(|e| e.to_string())?;
        Ok(namespaces)
    }


    pub fn get_hbase_table_list(&self, ns :&str) -> Result<Vec<HbaseTableStatus>, String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

        let tables_java_instance = self
            .jvm
            .invoke(
                &self.hbase_tool,
                "getTables",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(ns).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        let tables: Vec<HbaseTableStatus> = self
            .jvm
            .to_rust(tables_java_instance)
            .map_err(|e| e.to_string())?;
        Ok(tables)
    }

    pub fn get_hbase_table_data_list(&self,tablename :&str,page_num :i32, page_size:i32) -> Result<Vec<HashMap<String,String>>, String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        
        let data_java_instance = self
            .jvm
            .invoke(
                &self.hbase_tool,
                "getTableDataList",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(tablename).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(page_num).map_err(|e| e.to_string())?.into_primitive().map_err(|e| e.to_string())?,
                    InvocationArg::try_from(page_size).map_err(|e| e.to_string())?.into_primitive().map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        let data: Vec<HashMap<String,String>> = self
            .jvm
            .to_rust(data_java_instance)
            .map_err(|e| e.to_string())?;
        Ok(data)
    }

    pub fn get_hbase_table_data_count(&self,tablename :&str) -> Result<i64, String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

        let data_java_instance = self
            .jvm
            .invoke(
                &self.hbase_tool,
                "getTableDataCount",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(tablename).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        let data: i64 = self
            .jvm
            .to_rust(data_java_instance)
            .map_err(|e| e.to_string())?;
        Ok(data)
    }

    pub fn create_table(&self,settings :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "createTable",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(settings).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }


    pub fn create_namespace(&self,namespace :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "createNamespace",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(namespace).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete_table(&self,tablename :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "deleteTable",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(tablename).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete_namespace(&self,namespace :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "deleteNamespace",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(namespace).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }


    pub fn enable_table(&self,tablename :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "enableTable",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(tablename).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }


    pub fn disable_table(&self,tablename :&str) -> Result<(), String> {
        let conf_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_conf_map.clone(),
            )
            .map_err(|e| e.to_string())?;
        let env_java_map = self
            .jvm
            .java_map(
                JavaClass::String,
                JavaClass::String,
                self.hbase_env_map.clone(),
            )
            .map_err(|e| e.to_string())?;

         self
            .jvm
            .invoke(
                &self.hbase_tool,
                "disableTable",
                &[
                    InvocationArg::try_from(conf_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(env_java_map).map_err(|e| e.to_string())?,
                    InvocationArg::try_from(tablename).map_err(|e| e.to_string())?,
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

//hbase namespace 列表
#[tauri::command]
pub async fn get_hbase_namespace_list_command(id: i64) -> Result<Vec<String>, String> {
    let oper = get_hbase_oper(id)?;
    let namespaces = oper.get_hbase_namespace_list()?;
    Ok(namespaces)
}

//hbase 指定namespace下 table 列表
#[tauri::command]
pub async fn get_hbase_table_list_command(id: i64, namespace :&str) -> Result<Vec<HbaseTableStatus>, String> {
    let oper = get_hbase_oper(id)?;
    let tables = oper.get_hbase_table_list(namespace)?;
    Ok(tables)
}


#[tauri::command]
pub async fn get_hbase_table_data_list_command(id: i64, tablename :&str,page_num :i32, page_size:i32) -> Result<Vec<HashMap<String,String>>, String> {
    let oper = get_hbase_oper(id)?;
    let data = oper.get_hbase_table_data_list(tablename,page_num,page_size)?;
    Ok(data)
}

#[tauri::command]
pub async fn get_hbase_table_data_count_command(id: i64, tablename :&str) -> Result<i64, String> {
    let oper = get_hbase_oper(id)?;
    let data = oper.get_hbase_table_data_count(tablename)?;
    Ok(data)
}

#[tauri::command]
pub async fn create_table_command(id: i64, settings :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.create_table(settings)?;
    Ok(())
}
#[tauri::command]
pub async fn create_namespace_command(id: i64, namespace :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.create_namespace(namespace)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_namespace_command(id: i64, namespace :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.delete_namespace(namespace)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_table_command(id: i64, tablename :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.delete_table(tablename)?;
    Ok(())
}


#[tauri::command]
pub async fn enable_table_command(id: i64, tablename :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.enable_table(tablename)?;
    Ok(())
}




#[tauri::command]
pub async fn disable_table_command(id: i64, tablename :&str) -> Result<(), String> {
    let oper = get_hbase_oper(id)?;
    oper.disable_table(tablename)?;
    Ok(())
}



