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
          </tr>
        </table>
      </el-header>
      <el-main>
        <el-table :data="data" style="width: 100%">
          <el-table-column prop="name" label="Namespace" width="300">
          </el-table-column>
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
import { Namespace, get_hbase_namespace_list } from "../api/hbase_api.ts";
const router = useRouter();
const route = useRoute();

const data = ref<Namespace[]>([]);
 get_hbase_namespace_list(
  parseInt(route.params.id as string)
).then((res) => {
  data.value = res;
});


const backToHome = () => {
  router.push("/");
};
//返回历史上一页
const backToLastPage = () => {
  router.go(-1);
};
</script>
<style scoped></style>
