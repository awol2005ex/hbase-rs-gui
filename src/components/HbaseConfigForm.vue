<template>
 <el-form :model="hbaseConfigForm" label-width="150px" size="small" @submit.native.prevent>
    <el-form-item label="Name:">
      <input  
        style="width: 300px"
        clearable
        v-model="hbaseConfigForm.name"
      />
    </el-form-item>
    <el-form-item label="Hbase Envs:">
      <textarea  
        style="width: 300px"
        clearable
        v-model="hbaseConfigForm.hbase_env"
      />
    </el-form-item>
    <el-form-item label="Hbase Config:">
      <textarea  
        style="width: 300px"
        clearable
        v-model="hbaseConfigForm.hbase_config"
      />
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { Reactive, reactive, defineExpose } from "vue";
import { HbaseConfig } from "../api/hbase_config.ts";
const hbaseConfigForm :Reactive<HbaseConfig> = reactive({ 
  id: 0,  
  name: '',
  hbase_config: '{}',
  hbase_env: '{}',
  del_flag:0,
 });


const setHbaseConfigForm = (hbaseConfig: HbaseConfig) => {
  hbaseConfigForm.id = hbaseConfig.id;
  hbaseConfigForm.name = hbaseConfig.name;
  hbaseConfigForm.hbase_config = hbaseConfig.hbase_config;
  hbaseConfigForm.hbase_env = hbaseConfig.hbase_env;
}
defineExpose({
  hbaseConfigForm,setHbaseConfigForm
})

</script>

<style scoped></style>