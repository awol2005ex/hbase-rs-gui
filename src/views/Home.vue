<template>
  <div class="common-layout">
    <el-container>
      <el-main>
        <div class="flex flex-wrap gap-4">
          <el-card
            style="width: 480px; margin-top: 20px"
            shadow="always"
            v-for="item in hbaseConfigList"
            :key="item.id"
          >
            <table>
              <tr>
                <td>{{ item.name }}</td>
                <td>
                  <el-button
                    type="primary"
                    :icon="EditPen"
                    circle
                    @click="editHbaseConfig(item.id || 0)"
                    title="Edit"
                  />
                </td>
                <td>
                  <el-button
                    type="danger"
                    :icon="Delete"
                    circle
                    @click="removeHbaseConfig(item.id || 0)"
                    title="Delete"
                  />
                </td>
                <td>
                  <el-button
                    type="primary"
                    :icon="Connection"
                    circle
                    @click="connectToHbase(item.id || 0,item.name||'')"
                    title="Connect"
                  />
                </td>
              </tr>
            </table>
          </el-card>
          <el-card style="width: 480px; margin-top: 20px" shadow="always">
            <el-button
              type="primary"
              :icon="DocumentAdd"
              circle
              @click="addHbaseConfig"
            />
          </el-card>
        </div>
      </el-main>
    </el-container>
  </div>

  <el-dialog
    v-model="AddHbaseConfigDialogVisible"
    title="Add Hbase Config"
    width="500"
  >
    <HbaseConfigForm ref="hbaseConfigForm" />
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="AddHbaseConfigDialogVisible = false"
          >Cancel</el-button
        >
        <el-button type="primary" @click="AddHbaseConfigConfirm">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { Ref, ref, nextTick, watch } from "vue";
import {
  HbaseConfig,
  getHbaseConfigList,
  saveHbaseConfig,
  getHbaseConfig,
  deleteHbaseConfig,
  initConnection,
} from "../api/hbase_config.ts";
import HbaseConfigForm from "../components/HbaseConfigForm.vue";
import {
  DocumentAdd,
  EditPen,
  Delete,
  Connection,
} from "@element-plus/icons-vue";
import { useRouter, useRoute } from "vue-router";
import { ElMessage } from "element-plus";
import { getCurrentWindow } from "@tauri-apps/api/window";

const router = useRouter();
const route = useRoute();
//Hbase配置列表
const hbaseConfigList: Ref<Array<HbaseConfig>> = ref([]);

const refreshList = () => {
  getHbaseConfigList().then((res) => {
    hbaseConfigList.value = res;
  });
};
//初始化已保存的hbase配置列表
refreshList();
//打开新建窗口
const AddHbaseConfigDialogVisible: Ref<Boolean> = ref(false);

const addHbaseConfig = async () => {
  AddHbaseConfigDialogVisible.value = true;
  await nextTick();
  if (hbaseConfigForm.value) {
    hbaseConfigForm.value.setHbaseConfigForm({
      id: 0,
      name: "",
      hbase_config: "{}",
      hbase_env: "{}",
      del_flag: 0,
    });
  }
};

const hbaseConfigForm = ref<InstanceType<typeof HbaseConfigForm>>();
//保存
const AddHbaseConfigConfirm = () => {
  saveHbaseConfig(hbaseConfigForm.value?.hbaseConfigForm || {})
    .then(() => {
      AddHbaseConfigDialogVisible.value = false;
      refreshList();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        message: err.toString(),
        type: "error",
      });
    });
};

const editHbaseConfig = async (id: number) => {
  AddHbaseConfigDialogVisible.value = true;
  await nextTick();
  getHbaseConfig(id)
    .then((res) => {
      if (hbaseConfigForm.value) {
        hbaseConfigForm.value.setHbaseConfigForm(res);
      }
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        message: err.toString(),
        type: "error",
      });
    });
};
//删除hbase配置
const removeHbaseConfig = (id: number) => {
  deleteHbaseConfig(id)
    .then(() => {
      refreshList();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        message: err.toString(),
        type: "error",
      });
    });
};
//连接到hbase
const connectToHbase = async (id: number, name: string) => {
  try {
    getCurrentWindow().setTitle("Hbase Gui-[" + name + "]");
    //进入页面
    //router.push("/HbaseNamespaceListView/" + id );
  } catch (err: any) {
    ElMessage({
      showClose: true,
      message: err.toString(),
      type: "error",
    });
  }
};

// 监听路由变化
watch(
  () => route,
  (newRoute, oldRoute) => {
    console.log('Route changed:', newRoute, oldRoute);
    getCurrentWindow().setTitle("Hbase Gui");
  },
  { deep: true, immediate: true }
);
</script>

<style scoped></style>
