import { createWebHistory, createRouter } from "vue-router";

import About from "./views/AboutView.vue";
import Home from "./views/HomeView.vue";
import Profile from "./views/ProfileView.vue";
import ProfileInfo from "./components/profile/ProfileInfo.vue";
import ProfileEdit from "./components/profile/ProfileEdit.vue";

const routes = [
    { path: "/", component: Home, name: "home" },
    { path: "/about", component: About, name: "about" },
    {
        path: "/profile/:id",
        name: "profile",
        component: Profile,
        children: [
            {
                path: "info",
                name: "profile-info",
                component: ProfileInfo,
            },
            {
                path: "edit",
                name: "profile-edit",
                component: ProfileEdit,
            },
        ],
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
