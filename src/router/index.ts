// src/router/index.ts
import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
//首页
import Home from "../views/Home.vue";
import HbaseNamespaceListView from "../views/HbaseNamespaceListView.vue";
import HbaseNamespaceTableListView from "../views/HbaseNamespaceTableListView.vue";
import HbaseNamespaceTableDataListView from "../views/HbaseNamespaceTableDataListView.vue";
const routes: Array<RouteRecordRaw> = [
  { path: "/", name: "Home", component: Home },
  {
    path: "/HbaseNamespaceListView/:id",
    name: "HbaseNamespaceListView",
    component: HbaseNamespaceListView,
  },
  {
    path: "/HbaseNamespaceTableListView/:id/:namespace",
    name: "HbaseNamespaceTableListView",
    component: HbaseNamespaceTableListView,
  },
  {
    path: "/HbaseNamespaceTableDataListView/:id/:namespace/:tablename",
    name: "HbaseNamespaceTableDataListView",
    component: HbaseNamespaceTableDataListView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
