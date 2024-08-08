import { createWebHistory, createRouter } from "vue-router";

import About from "./views/AboutView.vue";
import SingleRecipeView from "./views/SingleRecipeView.vue";
import RecipesView from "./views/RecipesView.vue";
import Home from "./views/HomeView.vue";
import ProfileView from "./views/ProfileView.vue";
import ProfileInfo from "./components/profile/ProfileInfo.vue";
import ProfileEdit from "./components/profile/ProfileEdit.vue";
import NewRecipeView from "./views/NewRecipeView.vue";

const routes = [
    { path: "/", component: Home, name: "home" },
    { path: "/about", component: About, name: "about" },
    { path: "/new-recipe", component: NewRecipeView, name: "new-recipe" },
    {
        path: "/profile/:id",
        name: "profile",
        component: ProfileView,
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
    {
        path: "/recipes/:type",
        name: "recipes",
        component: RecipesView,
    },
    {
        path: "/recipe/:name",
        name: "single-recipe",
        component: SingleRecipeView,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) {
            return new Promise((resolve) => {
                setTimeout(() => {
                    resolve({
                        ...savedPosition,
                        behavior: "smooth",
                    });
                }, 100); // Small delay to ensure the new component is mounted
            });
        } else {
            return new Promise((resolve, reject) => {
                setTimeout(() => {
                    resolve({ left: 0, top: 0, behavior: "smooth" });
                }, 200);
            });
        }
    },
});

export default router;
