<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <table>
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
                  title="Create Table" /><el-button
                  type="primary"
                  :icon="Delete"
                  circle
                  @click="DeleteTables"
                  title="Delete Tables" /><el-button
                  type="primary"
                  :icon="Check"
                  circle
                  @click="EnableTables"
                  title="Enable Tables" /><el-button
                  type="primary"
                  :icon="Close"
                  circle
                  @click="DisableTables"
                  title="Disable Tables"
              /></el-button-group>
            </td>

            <td>
              <el-switch
                style="float: left"
                v-model="show_table_metrics"
                inline-prompt
                size="large"
                @change="on_show_table_metrics_change"
                active-text="show table metrics"
                inactive-text="don't show table metrics"
              />
            </td>
          </tr>
        </table>
      </el-header>
      <el-main>
        <el-table
          :data="data"
          style="width: 100%"
          @selection-change="handleSelectionChange"
        >
          <el-table-column type="selection" width="55" />
          <el-table-column prop="name" label="Table" width="300">
            <template #default="scope">
              <el-link
                :underline="false"
                @click="goToTableDataListView(scope.row)"
                >{{ scope.row.name }}</el-link
              >
            </template>
          </el-table-column>
          <el-table-column
            prop="enabled"
            label="Enabled"
            width="300"
          ></el-table-column>

          <el-table-column
            v-if="show_table_metrics"
            prop="disksize"
            label="DiskSize"
            width="300"
          >
            <template #default="scope">
              {{ formatFileSize(scope.row.disksize) }}
            </template>
          </el-table-column>
          <el-table-column
            v-if="show_table_metrics"
            prop="memstoresize"
            label="MemstoreSize"
            width="300"
            ><template #default="scope">
              {{ formatFileSize(scope.row.memstoresize) }}
            </template></el-table-column
          >
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
import { ElMessage, ElLoading, ElMessageBox } from "element-plus";
import {
  Back,
  HomeFilled,
  Plus,
  Delete,
  Close,
  Check,
} from "@element-plus/icons-vue";
import {
  HbaseTableStatus,
  get_hbase_table_list,
  create_table,
  delete_table,
  enable_table,
  disable_table,
  get_hbase_table_metrics_list,
} from "../api/hbase_api.ts";
const router = useRouter();
const route = useRoute();

const data = ref<HbaseTableStatus[]>([]);

const show_table_metrics = ref(false);
//显示文件大小
const formatFileSize = (size: number) => {
  if (size < 1024) {
    return size + " MB";
  } else if (size < 1024 * 1024) {
    return (size / 1024).toFixed(2) + " GB";
  } else if (size < 1024 * 1024 * 1024) {
    return (size / 1024 / 1024).toFixed(2) + " TB";
  } else if (size < 1024 * 1024 * 1024 * 1024) {
    return (size / 1024 / 1024 / 1024).toFixed(2) + " PB";
  } else {
    return size + " MB";
  }
};

const refresh = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });

  if (show_table_metrics.value) {
    get_hbase_table_metrics_list(
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
  } else {
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
  }
};

refresh();

const on_show_table_metrics_change = () => {
  refresh();
};
const backToHome = () => {
  router.push("/");
};
//返回历史上一页
const backToLastPage = () => {
  router.go(-1);
};

//跳转到table data列表
const goToTableDataListView = (row: HbaseTableStatus) => {
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
    .then((_res) => {
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

//多选
const multipleSelection = ref<HbaseTableStatus[]>([]);
const handleSelectionChange = (val: HbaseTableStatus[]) => {
  multipleSelection.value = val;
};

const DeleteTables = async () => {
  if (multipleSelection.value.length == 0) {
    ElMessage({
      showClose: true,
      message: "Please select tables",
      type: "error",
    });
    return;
  }
  for (let element of multipleSelection.value) {
    if (element.enabled) {
      ElMessage({
        showClose: true,
        message:
          "Table [" + element.name + "] is enabled, please disable it first",
        type: "error",
      });
      return;
    }
  }
  const s2 = await ElMessageBox.confirm("Delete seleced Tables?", "Warning", {
    confirmButtonText: "OK",
    cancelButtonText: "Cancel",
    type: "warning",
    draggable: true,
  });
  if (s2.action != "confirm") {
    const loadingInstance1 = ElLoading.service({ fullscreen: true });
    try {
      for (let element of multipleSelection.value) {
        if (element.name) {
          await delete_table(parseInt(route.params.id as string), element.name);
        }
      }
    } catch (error: any) {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
    } finally {
      loadingInstance1.close();
      refresh();
    }
  } else {
    return;
  }
};

const EnableTables = async () => {
  if (multipleSelection.value.length == 0) {
    ElMessage({
      showClose: true,
      message: "Please select tables",
      type: "error",
    });
    return;
  }
  for (let element of multipleSelection.value) {
    if (element.enabled) {
      ElMessage({
        showClose: true,
        message:
          "Table [" + element.name + "] is enabled, no need to enable again",
        type: "error",
      });
      return;
    }
  }
  const s2 = await ElMessageBox.confirm("Enable seleced Tables?", "Warning", {
    confirmButtonText: "OK",
    cancelButtonText: "Cancel",
    type: "warning",
    draggable: true,
  });
  if (s2.action != "confirm") {
    const loadingInstance1 = ElLoading.service({ fullscreen: true });
    try {
      for (let element of multipleSelection.value) {
        if (element.name) {
          await enable_table(parseInt(route.params.id as string), element.name);
        }
      }
    } catch (error: any) {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
    } finally {
      loadingInstance1.close();
      refresh();
    }
  } else {
    return;
  }
};

const DisableTables = async () => {
  if (multipleSelection.value.length == 0) {
    ElMessage({
      showClose: true,
      message: "Please select tables",
      type: "error",
    });
    return;
  }
  for (let element of multipleSelection.value) {
    if (!element.enabled) {
      ElMessage({
        showClose: true,
        message:
          "Table [" + element.name + "] is disabled, no need to disable again",
        type: "error",
      });
      return;
    }
  }
  const s2 = await ElMessageBox.confirm("Disable seleced Tables?", "Warning", {
    confirmButtonText: "OK",
    cancelButtonText: "Cancel",
    type: "warning",
    draggable: true,
  });
  if (s2.action != "confirm") {
    const loadingInstance1 = ElLoading.service({ fullscreen: true });
    try {
      for (let element of multipleSelection.value) {
        if (element.name) {
          await disable_table(
            parseInt(route.params.id as string),
            element.name
          );
        }
      }
    } catch (error: any) {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
    } finally {
      loadingInstance1.close();
      refresh();
    }
  } else {
    return;
  }
};
</script>
<style scoped></style>
