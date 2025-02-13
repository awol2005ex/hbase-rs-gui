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
                  title="Back To Last Page"
              /></el-button-group>
            </td>

            <td>
              {{ route.params.tablename }}
            </td>
          </tr>
        </table>
      </el-header>
      <el-main>
        <el-table :data="data" style="width: 100%">
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
      <el-footer>
        <table>
          <tr>
            <td>Current Page {{ currentPage }}</td>
            <td>
              <el-button type="primary" @click="NextPage()"
                >Next Page</el-button
              >
            </td>
            <td>
              <el-button type="primary" @click="PrevPage()"
                >Prev Page</el-button
              >
            </td>
            <td>Total:{{ total==-1 ?"?":total }}</td>

            <td>
              <el-button type="primary" @click="GetTotalCount()"
                >Get Total Count</el-button
              >
            </td>
          </tr>
        </table>
      </el-footer>
    </el-container>
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { ElMessage, ElLoading } from "element-plus";
import {
  Back,
  HomeFilled,
} from "@element-plus/icons-vue";

import {
  get_hbase_table_data_list,
  get_hbase_table_data_count,
} from "../api/hbase_api.ts";

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
const total = ref(-1);
const currentPage = ref(1);

const queryData = async () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  try {
    data.value = await get_hbase_table_data_list(
      parseInt(route.params.id as string),
      route.params.tablename as string,
      currentPage.value,
      pageSize.value
    );
  } catch (error: any) {
    ElMessage({
      showClose: true,
      message: error.toString(),
      type: "error",
    });
    loadingInstance1.close();
  }
  if (data.value.length > 0) {
    var columnSet: Set<string> = new Set();
    data.value.forEach((d) => {
      Object.keys(d)
        .filter((key) => key != "row")
        .forEach((key) => {
          columnSet.add(key.toString());
        });
    });
    columns.value = Array.from(columnSet);
  }

  loadingInstance1.close();
};


const NextPage = async () => {
  currentPage.value = currentPage.value + 1;
  await queryData();
};
const PrevPage = async () => {
  currentPage.value = currentPage.value - 1 < 1 ? 1 : currentPage.value - 1;
  await queryData();
};
queryData().then(() => {});

const GetTotalCount = async () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  try {
    total.value = await get_hbase_table_data_count(
      parseInt(route.params.id as string),
      route.params.tablename as string
    );
  } catch (error: any) {
    ElMessage({
      showClose: true,
     message: error.toString(),
    })
  }
  loadingInstance1.close();
}
</script>
<style scoped></style>
