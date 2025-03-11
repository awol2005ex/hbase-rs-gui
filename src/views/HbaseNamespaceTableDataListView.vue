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
                  title="Back To Home"
                />
                <el-button
                  type="primary"
                  :icon="Back"
                  circle
                  @click="backToLastPage"
                  title="Back To Last Page"
                />
                <el-button
                  type="primary"
                  :icon="Connection"
                  circle
                  @click="createTableSql"
                  title="Create Table SQL"
                />
              </el-button-group>
            </td>

            <td>
              {{ route.params.tablename }}
            </td>
            <td>Column Families: {{ columnFamilies.join(",") }}</td>
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
            <td>Total:{{ total == -1 ? "?" : total }}</td>

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

  <el-drawer v-model="show_sqlcreator" title="create table sql" size="80%">
    <el-tabs v-model="activeName" class="sql-tabs">
      <el-tab-pane label="Spark SQL TEMPORARY VIEW " name="spark">
        <pre
          style="white-space: pre-wrap; word-wrap: break-word; overflow: auto"
          >{{ spark_tv_sql }}</pre
        >
      </el-tab-pane>
      <el-tab-pane label="FLINK TABLE SQL " name="flink">
        <pre
          style="white-space: pre-wrap; word-wrap: break-word; overflow: auto"
          >{{ flink_sql }}</pre
        >
      </el-tab-pane>
      <el-tab-pane label="HIVE TABLE SQL " name="hive">
        <pre
          style="white-space: pre-wrap; word-wrap: break-word; overflow: auto"
          >{{ hive_sql }}</pre
        >
      </el-tab-pane>
    </el-tabs>
  </el-drawer>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { ElMessage, ElLoading } from "element-plus";
import { Back, HomeFilled, Connection } from "@element-plus/icons-vue";

import {
  get_hbase_table_data_list,
  get_hbase_table_data_count,
  get_hbase_table_column_family_list,
} from "../api/hbase_api.ts";

import {
  get_spark_tv_sql_impl,
  create_flink_table_sql_impl,
  create_hive_sql_impl,
} from "../utils/sql.ts";

const router = useRouter();
const route = useRoute();

const data = ref<Object[]>([]);

const columnFamilies = ref<string[]>([]);

get_hbase_table_column_family_list(
  parseInt(route.params.id as string),
  route.params.tablename as string
).then((res) => {
  columnFamilies.value = res;
});
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
    });
  }
  loadingInstance1.close();
};

const show_sqlcreator = ref(false);

const spark_tv_sql = ref("");
const flink_sql = ref("");
const hive_sql = ref("");

const get_spark_tv_sql = async () => {
  spark_tv_sql.value = await get_spark_tv_sql_impl(
    route.params.tablename as string,
    columns.value
  );
};

const create_flink_table_sql = async () => {
  flink_sql.value = await create_flink_table_sql_impl(
    route.params.tablename as string,
    columns.value,
    columnFamilies.value
  );
};

const create_hive_sql = async () => {
  hive_sql.value = await create_hive_sql_impl(
    route.params.tablename as string,
    columns.value
  );
};

const createTableSql = async () => {
  show_sqlcreator.value = true;
  await get_spark_tv_sql();
  await create_flink_table_sql();
  await create_hive_sql();
};

const activeName = ref("spark");
</script>
<style scoped></style>
