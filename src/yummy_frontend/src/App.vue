<script setup>
import { onMounted, ref } from "vue";
import Navigation from "./components/Nav.vue";
import LeftMenu from "./components/LeftMenu.vue";
import Recipes from "./components/Recipe.vue";
import MainHeader from "./components/MainHeader.vue";

const leftMenuIsVisible = ref(false);
function handleItemClick(item) {
    console.log("Clicked item:", item);
}
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
            class="realative app-content flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[80px]"
        >
            <Navigation class="mobile-view" @show-menu="leftMenuIsVisible = true" />
            <MainHeader />
            <main class="relative w-full bg-[#545454]">
                <h2>
                    <h2 class="p-4 text-center text-4xl text-white">Popular Recipes</h2>
                </h2>
                <Recipes @item-click="handleItemClick" />
                <!-- <img v-if="imageInfo" :src="imageInfo" alt="Fetched Image" /> -->
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
