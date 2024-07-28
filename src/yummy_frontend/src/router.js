import { createWebHistory, createRouter } from "vue-router";

import About from "./views/AboutView.vue";
import Home from "./views/HomeView.vue";

const routes = [
    { path: "/", component: Home },
    { path: "/about", component: About },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
