<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <table width="100%">
          <tr>
            <td>
              <el-button-group style="float: left">
                <el-button
                  type="primary"
                  :icon="HomeFilled"
                  circle
                  @click="backToHome"
                  title="Back To Home" />
                <el-button
                  type="primary"
                  :icon="Back"
                  circle
                  @click="backToLastPage"
                  title="Back To Last Page" /></el-button-group>
            </td>

            <td>
               {{ route.params.tablename}}
            </td>
          </tr>
        </table>
      </el-header>
      <el-main>
        <el-table
          :data="data"
          style="width: 100%"
        >
          <el-table-column prop="row" label="row" width="200" />
          <el-table-column
            v-for="(item, index) in columns"
            :key="index"
            :prop="item"
            :label="item"
            :width="200"
          />
        </el-table>

      </el-main>
    </el-container>
  </div>
</template>
<script setup lang="ts">
import { Ref, ref, nextTick, watch } from "vue";
import { useRouter, useRoute } from "vue-router";
import {
  Back,
  Refresh,
  Folder,
  Document,
  HomeFilled,
  Search,
  Location,
  Upload,
  Delete,
  FolderAdd,
  DocumentAdd,
  Suitcase,
  Download,
} from "@element-plus/icons-vue";

import { get_hbase_table_data_list,get_hbase_table_data_count } from "../api/hbase_api.ts";

const router = useRouter();
const route = useRoute();

const data = ref<Object[]>([]);




const backToHome = () => {
  router.push("/");
};
//返回历史上一页
const backToLastPage = () => {
  router.go(-1);
};
const columns = ref<string[]>([]);
const pageSize = ref(10);
const total = ref(0);
const currentPage = ref(1);

const queryData= async () => {
  data.value =await get_hbase_table_data_list( parseInt(route.params.id as string),(route.params.tablename as string),currentPage.value, pageSize.value )

  if(data.value.length>0) {
    columns.value =Object.keys(data["value"][0]).filter((key) => key!='row')
  }
}
const handleCurrentChange = async (val: number) => {
  currentPage.value = val;
  await queryData();
};
const handleSizeChange = async (val: number) => {
  pageSize.value = val;
  await queryData();
};

queryData().then(()=>{})

</script>
<style scoped></style>
