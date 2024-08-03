<script setup>
import { ref } from "vue";
import Navigation from "./components/Nav.vue";
import LeftMenu from "./components/LeftMenu.vue";

console.log("rendering app");
const leftMenuIsVisible = ref(false);
</script>

<template>
    <div class="main-container flex w-full flex-row">
        <div
            v-show="leftMenuIsVisible"
            class="mobile-view fixed z-[120] h-full w-full bg-black/40 opacity-100"
            @click="leftMenuIsVisible = false"
        ></div>
        <LeftMenu :isVisible="leftMenuIsVisible" />
        <div
            class="realative app-content flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[80px]"
        >
            <Navigation class="mobile-view" @show-menu="leftMenuIsVisible = true" />
            <main class="relative w-full bg-gray-800">
                <RouterView />
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
