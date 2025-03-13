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
                  title="Disable Tables" /><el-button
                  type="primary"
                  :icon="Collection"
                  circle
                  @click="ExportCreateExternalTableSQLtoExcel"
                  title="Export Create External Table SQL to Excel"
              /><el-button
                  type="primary"
                  :icon="List"
                  circle
                  @click="ExportTablesToExcel"
                  title="Export Table List To Excel"
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

            <td>
              <el-input
                v-model="search_words"
                style="
                  width: 240px;
                  float: left;
                  margin-left: 10px;
                  margin-top: 5px;
                "
                placeholder="Search Namespaces"
                :prefix-icon="Search"
                @change="on_search_words_change"
                clearable
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
          @sort-change="sortChange"
        >
          <el-table-column type="selection" width="55" />
          <el-table-column
            prop="name"
            label="Table"
            width="300"
            sortable="custom"
          >
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
            sortable="custom"
          ></el-table-column>

          <el-table-column
            v-if="show_table_metrics"
            prop="disksize"
            label="DiskSize"
            width="300"
            sortable="custom"
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
            sortable="custom"
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
  Search,
  Collection,
  List,
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

import { export_table_sql_to_excel } from "../utils/sql";
import { getHbaseConfig } from "../api/hbase_config.ts";
import writeXlsxFile from "write-excel-file";

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
const sortProp = ref("");
const sortOrder = ref("");
//排序
const sortChange = (row: { column: any; prop: any; order: any }) => {
  const { prop, order } = row;

  sortProp.value = prop;
  sortOrder.value = order;
  refresh();
};
const refresh = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });

  if (show_table_metrics.value) {
    get_hbase_table_metrics_list(
      parseInt(route.params.id as string),
      route.params.namespace as string
    )
      .then((res) => {
        if (search_words.value == "") {
          data.value = res;
        } else {
          data.value = res.filter((item) => {
            return item.name?.includes(search_words.value);
          });
        }
        data.value = data.value //排序
          .sort((a: HbaseTableStatus, b: HbaseTableStatus) => {
            const prop = sortProp.value as keyof HbaseTableStatus;
            const aVal = a[prop] ?? 0; // 处理undefined情况
            const bVal = b[prop] ?? 0;

            if (aVal < bVal) return sortOrder.value === "descending" ? -1 : 1;
            if (aVal > bVal) return sortOrder.value === "descending" ? 1 : -1;
            return 0;
          });
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
        if (search_words.value == "") {
          data.value = res;
        } else {
          data.value = res.filter((item) => {
            return item.name?.includes(search_words.value);
          });
        }
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

const search_words = ref("");
const on_search_words_change = () => {
  refresh();
};

const ExportCreateExternalTableSQLtoExcel = async () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  try {
    const config = await getHbaseConfig(parseInt(route.params.id as string));
    await export_table_sql_to_excel(
      parseInt(route.params.id as string),
      multipleSelection.value,
      new Map(Object.entries(JSON.parse(config.hbase_config || "{}")))
    );
  } catch (error: any) {
    ElMessage({
      showClose: true,
      message: error.toString(),
      type: "error",
    });
  }

  loadingInstance1.close();
};


const  ExportTablesToExcel = async () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  try {
    let schema = [
      {
        column: "Table",
        type: String,
        value: (s: { [x: string]: any; }) => s["name"],
      },
      {
        column: "Enabled",
        type: String,
        value: (s: { [x: string]: any; }) => s["enabled"].toString(),
      },
    ];
    if(show_table_metrics.value){
      schema.push({
        column: "Disk Size",
        type: String,
        value: (s: { [x: string]: any; }) => formatFileSize(s["disksize"]),
      });
      schema.push({
        column: "Mem Store Size",
        type: String,
        value: (s: { [x: string]: any; }) => formatFileSize(s["memstoresize"]),
      });
    }
    await writeXlsxFile( data.value, {
      schema,
      fileName: "tables.xlsx",
    });
  } catch (error: any) {
    ElMessage({
      showClose: true,
      message: error.toString(),
      type:"error",
    })
  }
  loadingInstance1.close();

}
</script>
<style scoped></style>
