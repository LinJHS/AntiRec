import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    children: [
      { path: "/", redirect: "/home" },
      {
        path: "/home",
        name: "home",
        component: () => import("../views/HomeView.vue"),
      },
      {
        path: "/protect",
        meta: { requiresAuth: true },
        name: "protect",
        component: () => import("../views/ProtectView.vue"),
      },
      {
        path: "/list",
        meta: { requiresAuth: true },
        name: "list",
        component: () => import("../views/ListView.vue"),
      },
      {
        path: "/async",
        meta: { requiresAuth: true },
        name: "async",
        component: () => import("../views/AsyncView.vue"),
      },
      {
        path: "/login",
        name: "login",
        component: () => import("../views/LoginView.vue"),
      },
      {
        path: "/register",
        name: "register",
        component: () => import("../views/RegisterView.vue"),
      },
    ],
  },
  {
    path: "/404",
    name: "404",
    component: () => import("../views/NotFound.vue"),
  },
  // 所有未定义路由，全部重定向到 404 页
  { path: "/:pathMatch(.*)*", redirect: "/404" },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

router.beforeEach((to, _from) => {
  if (to.meta.requiresAuth) {
    if (localStorage.getItem("Token") === null) {
      return {
        path: "/login",
      };
    }
  }
});

export default router;
