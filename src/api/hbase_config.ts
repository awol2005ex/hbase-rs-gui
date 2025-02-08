import { invoke } from "@tauri-apps/api/core";
//HDFS连接配置列表
export interface HbaseConfig {
  id?: number;
  name?: string;
  hbase_config?: string; //json其他配置
  hbase_env?: string;
  del_flag?: number; //0正常 1删除
}
//获取HDFS连接配置列表
export const getHbaseConfigList = async () => {
  const result: Array<HbaseConfig> = await invoke("get_hbase_config_list", {});
  return result;
};

//保证HDFS连接配置
export const saveHbaseConfig = async (hbase_config: HbaseConfig) => {
  await invoke("save_hbase_config", { hdfsConfig: hbase_config });
};

//获取HDFS连接配置列表
export const getHbaseConfig = async (id: number) => {
  const result: HbaseConfig = await invoke("get_hbase_config", { id: id });
  return result;
};
//删除HDFS连接配置
export const deleteHbaseConfig = async (id: number) => {
  await invoke("delete_hbase_config", { id: id });
};


//初始化连接
export const initConnection = async (id: number) => {
  await invoke("init_connection", { id: id });
};
