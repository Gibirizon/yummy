<script setup>
import { onMounted, ref } from "vue";
import Navigation from "./components/Nav.vue";
import LeftMenu from "./components/LeftMenu.vue";
import Recipes from "./components/Recipe.vue";

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
            class="realative app-content flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[70px]"
        >
            <Navigation class="mobile-view" @show-menu="leftMenuIsVisible = true" />
            <main class="relative w-full bg-[#282828]">
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
