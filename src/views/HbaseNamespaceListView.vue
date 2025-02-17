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
                /><el-button
                  type="primary"
                  :icon="Plus"
                  circle
                  @click="CreateHbaseNamespaceDialogVisible = true"
                  title="Create Namespace"
                />
                <el-button
                  type="primary"
                  :icon="Delete"
                  circle
                  @click="DeleteNamespaces"
                  title="Delete Namespaces"
                />
              </el-button-group>
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
          <el-table-column prop="name" label="Namespace" width="300">
            <template #default="scope">
              <el-link
                :underline="false"
                @click="goToTableListView(scope.row)"
                >{{ scope.row.name }}</el-link
              >
            </template>
          </el-table-column>
        </el-table>
      </el-main>
    </el-container>
  </div>

  <el-dialog
    v-model="CreateHbaseNamespaceDialogVisible"
    title="Create Hbase Namespace"
    width="500"
  >
    <el-form
      :model="createHbaseNamespaceForm"
      label-width="150px"
      size="small"
      @submit.native.prevent
    >
      <el-form-item label="namespace:">
        <textarea
          style="width: 300px"
          clearable
          v-model="createHbaseNamespaceForm.namespace"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="CreateHbaseNamespaceDialogVisible = false"
          >Cancel</el-button
        >
        <el-button type="primary" @click="CreateHbaseNamespaceConfirm">
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
import { Back, HomeFilled, Delete, Plus } from "@element-plus/icons-vue";
import {
  Namespace,
  get_hbase_namespace_list,
  create_namespace,
  delete_namespace,
} from "../api/hbase_api.ts";
const router = useRouter();
const route = useRoute();

const data = ref<Namespace[]>([]);
const refresh = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  get_hbase_namespace_list(parseInt(route.params.id as string))
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
//跳转到table列表
const goToTableListView = (row: Namespace) => {
  router.push(
    "/HbaseNamespaceTableListView/" +
      (route.params.id as string) +
      "/" +
      row.name
  );
};

const CreateHbaseNamespaceDialogVisible = ref(false);
const createHbaseNamespaceForm = ref({
  namespace: "",
});
const CreateHbaseNamespaceConfirm = () => {
  const loadingInstance1 = ElLoading.service({ fullscreen: true });
  create_namespace(
    parseInt(route.params.id as string),
    createHbaseNamespaceForm.value.namespace
  )
    .then((_res) => {
      ElMessage({
        showClose: true,
        message: "Create Hbase Namespace Success",
        type: "success",
      });
      CreateHbaseNamespaceDialogVisible.value = false;
    })
    .catch((error) => {
      ElMessage({
        showClose: true,
        message: error.toString(),
        type: "error",
      });
    })
    .finally(() => {
      loadingInstance1.close();
      refresh();
    });
};

//多选
const multipleSelection = ref<Namespace[]>([]);
const handleSelectionChange = (val: Namespace[]) => {
  multipleSelection.value = val;
};

const DeleteNamespaces = async () => {
  if (multipleSelection.value.length == 0) {
    ElMessage({
      showClose: true,
      message: "Please select namespaces",
      type: "error",
    });
    return;
  }
  const s2 = await ElMessageBox.confirm(
    "Delete seleced Namespaces?",
    "Warning",
    {
      confirmButtonText: "OK",
      cancelButtonText: "Cancel",
      type: "warning",
      draggable: true,
    }
  );
  if (s2.action != "confirm") {
    const loadingInstance1 = ElLoading.service({ fullscreen: true });
    try {
      for (let element of multipleSelection.value) {
        if(element.name){
        await delete_namespace(
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
