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
                  title="Back To Last Page" /><el-button
                  type="primary"
                  :icon="Plus"
                  circle
                  @click="CreateHbaseTableDialogVisible = true"
                  title="Create Table"
              /></el-button-group>
            </td>
          </tr>
        </table>
      </el-header>
      <el-main>
        <el-table :data="data" style="width: 100%">
          <el-table-column prop="name" label="Table" width="300">
            <template #default="scope">
              <el-link
                :underline="false"
                @click="goToTableDataListView(scope.row)"
                >{{ scope.row.name }}</el-link
              >
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
  </div>

  <el-dialog
    v-model="CreateHbaseTableDialogVisible"
    title="Create Hbase Table"
    width="500"
  >
    <el-form
      :model="createHbaseTableForm"
      label-width="150px"
      size="small"
      @submit.native.prevent
    >
      <el-form-item label="Settings:">
        <textarea
          style="width: 300px"
          clearable
          v-model="createHbaseTableForm.settings"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="CreateHbaseTableDialogVisible = false"
          >Cancel</el-button
        >
        <el-button type="primary" @click="CreateHbaseTableConfirm">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { ElMessage, ElLoading } from "element-plus";
import { Back, HomeFilled, Plus } from "@element-plus/icons-vue";
import {
  Namespace,
  get_hbase_table_list,
  create_table,
} from "../api/hbase_api.ts";
const router = useRouter();
const route = useRoute();

const data = ref<Namespace[]>([]);

const refresh = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  get_hbase_table_list(
    parseInt(route.params.id as string),
    route.params.namespace as string
  )
    .then((res) => {
      data.value = res;
      loadingInstance1.close();
    })
    .catch((error) => {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
      loadingInstance1.close();
    });
};

refresh();
const backToHome = () => {
  router.push("/");
};
//返回历史上一页
const backToLastPage = () => {
  router.go(-1);
};

//跳转到table data列表
const goToTableDataListView = (row: Namespace) => {
  router.push(
    "/HbaseNamespaceTableDataListView/" +
      (route.params.id as string) +
      "/" +
      (route.params.namespace as string) +
      "/" +
      row.name
  );
};

//创建表
const CreateHbaseTableDialogVisible = ref(false);
const CreateHbaseTableConfirm = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  create_table(
    parseInt(route.params.id as string),
    createHbaseTableForm.value.settings
  )
    .then((res) => {
      ElMessage({
        showClose: true,
        message: "Create Table Success",
        type: "success",
      });
      loadingInstance1.close();
      CreateHbaseTableDialogVisible.value = false;
      refresh();
    })
    .catch((error) => {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
      loadingInstance1.close();
    });
};

const createHbaseTableForm = ref({
  settings:
    '{"tableName":"' +
    (route.params.namespace as string) +
    ':[TableName]", "columnFamilies":[{"name":"[ColumnFamilyName]", "maxVersions":1}]}',
});
</script>
<style scoped></style>
