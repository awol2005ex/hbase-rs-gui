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