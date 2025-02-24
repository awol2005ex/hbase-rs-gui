import { invoke } from "@tauri-apps/api/core";


export class Namespace {
   
    name?: string; 

    disksize? :number

    memstoresize? :number

    constructor(name :string) { 
      this.name = name;
    }
  }
//获取hbase namespace列表
export const get_hbase_namespace_list = async (id :number) => {
  const nameList: Array<Namespace> = await invoke("get_hbase_namespace_list_command", {id});
  return nameList;
};

export const get_hbase_namespace_metrics_list = async (id :number) => {
  const nameList: Array<Namespace> = await invoke("get_hbase_namespace_metrics_list_command", {id});
  return nameList;
};

export class HbaseTableStatus {
   
  name?: string; 

  namespace? :string

  enabled? :boolean

  disksize? :number

  memstoresize? :number

  constructor(name :string,namespace :string ,enabled: boolean) { 
    this.name = name;
    this.enabled = enabled;
    this.namespace = namespace;
  }
}
//获取hbase namespace下table 列表
export const get_hbase_table_list = async (id :number, namespace :string) => {
  const tableList: Array<HbaseTableStatus> = await invoke("get_hbase_table_list_command", {id:id,namespace:namespace});
  
  return tableList;
};

export const get_hbase_table_metrics_list = async (id :number, namespace :string) => {
  const tableList: Array<HbaseTableStatus> = await invoke("get_hbase_table_metrics_list_command", {id:id,namespace:namespace});
  
  return tableList;
};

export const get_hbase_table_data_list = async (id :number, tablename :string ,pageNum :number,pageSize:number) => {
  const data: Array<Object> = await invoke("get_hbase_table_data_list_command", {id:id,tablename:tablename,pageNum:pageNum,pageSize:pageSize});
  
  return data;
};

export const get_hbase_table_data_count = async (id :number, tablename :string ) => {
  const c :number = await invoke("get_hbase_table_data_count_command", {id:id,tablename:tablename});
  
  return c;
};

export const create_table = async (id :number, settings :string ) => {
  await invoke("create_table_command", {id:id,settings:settings});
};
export const create_namespace = async (id :number, namespace :string ) => {
  await invoke("create_namespace_command", {id:id,namespace:namespace});
};
export const delete_namespace = async (id :number, namespace :string ) => {
  await invoke("delete_namespace_command", {id:id,namespace:namespace});
};

export const delete_table = async (id :number, tablename :string ) => {
  await invoke("delete_table_command", {id:id,tablename:tablename});
};
export const enable_table = async (id :number, tablename :string ) => {
  await invoke("enable_table_command", {id:id,tablename:tablename});
};
export const disable_table = async (id :number, tablename :string ) => {
  await invoke("disable_table_command", {id:id,tablename:tablename});
};