<script setup>
import { onMounted, ref } from "vue";
import Navigation from "./components/Nav.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import LeftMenu from "./components/LeftMenu.vue";
import Recipes from "./components/Recipe.vue";

const popular_recipes = ref([]);
async function getRecipes() {
    await yummy_backend
        .popular_recipes()
        .then((response) => {
            const res = JSON.parse(response);
            popular_recipes.value = res.data.popularRecipes.edges;
            console.log(popular_recipes.value);
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}

const leftMenuIsVisible = ref(false);

function handleItemClick(item) {
    console.log("Clicked item:", item);
}
onMounted(() => {
    getRecipes();
});
</script>

<template>
    <div class="main-container flex w-full flex-row">
        <div
            v-if="leftMenuIsVisible"
            class="mobile-view fixed z-[120] h-full w-full bg-black/40 opacity-100"
            @click="leftMenuIsVisible = false"
        ></div>
        <LeftMenu :isVisible="leftMenuIsVisible" />
        <div
            class="realative app-content flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[70px]"
        >
            <Navigation class="mobile-view" @show-menu="leftMenuIsVisible = true" />
            <main class="relative w-full bg-[#282828]">
                <Recipes :items="popular_recipes" v-if="popular_recipes" @item-click="handleItemClick" />
            </main>
        </div>
    </div>
</template>

<style scoped>
@media screen and (min-width: 992px) {
    .app-content {
        @apply pt-0;
    }
    .mobile-view {
        @apply hidden;
    }
}
</style>
