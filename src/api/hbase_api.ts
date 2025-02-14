import { invoke } from "@tauri-apps/api/core";


export class Namespace {
   
    name?: string; 

    constructor(name :string) { 
      this.name = name;
    }
  }
//获取hbase namespace列表
export const get_hbase_namespace_list = async (id :number) => {
  const nameList: Array<string> = await invoke("get_hbase_namespace_list_command", {id});
  const namespaceList: Array<Namespace>= nameList.map(item=> new Namespace(item));
  return namespaceList;
};


export class HbaseTable {
   
  name?: string; 

  constructor(name :string) { 
    this.name = name;
  }
}
//获取hbase namespace下table 列表
export const get_hbase_table_list = async (id :number, namespace :string) => {
  const nameList: Array<string> = await invoke("get_hbase_table_list_command", {id:id,namespace:namespace});
  const tableList: Array<HbaseTable>= nameList.map(item=> new HbaseTable(item));
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