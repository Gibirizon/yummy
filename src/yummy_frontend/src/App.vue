<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import Nav from "./components/Nav.vue";
import LeftMenu from "./components/LeftMenu.vue";
import Search from "./components/Search.vue";

console.log("rendering app");
const isLeftMenuVisible = ref(false);
const isSearchVisible = ref(false);

function toggleSearch() {
    isSearchVisible.value = !isSearchVisible.value;
}
// Add event listener for Ctrl+K
onMounted(() => {
    window.addEventListener("keydown", (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === "k") {
            e.preventDefault();
            toggleSearch();
        }
    });
});

onUnmounted(() => {
    window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
    <Search :isVisible="isSearchVisible" @close="isSearchVisible = false" />
    <div class="main-container flex w-full flex-row">
        <LeftMenu :isVisible="isLeftMenuVisible" @toggle-search="toggleSearch" @close="isLeftMenuVisible = false" />
        <div
            class="realative app-content flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[80px]"
        >
            <Nav class="mobile-view" @toggle-search="toggleSearch" @show-menu="isLeftMenuVisible = true" />
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
