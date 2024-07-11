<script setup>
import { ref, onMounted, onUnmounted } from "vue";

const topNavBarIsPrimary = ref(true);
const widthGreaterThanMobile = ref(false);
let mql;

function toggleSideNavBar() {
    topNavBarIsPrimary.value = false;
}

function hideSideNavBar() {
    topNavBarIsPrimary.value = true;
}

function handleWidthChange(e) {
    widthGreaterThanMobile.value = e.matches;
    if (widthGreaterThanMobile.value) {
        topNavBarIsPrimary.value = false;
    }
}

onMounted(() => {
    mql = window.matchMedia("(min-width: 992px)");
    handleWidthChange(mql);
    mql.addEventListener("change", handleWidthChange);
});

onUnmounted(() => {
    mql.removeEventListener("change", handleWidthChange);
});
</script>

<template>
    <div
        :class="[
            topNavBarIsPrimary || widthGreaterThanMobile
                ? 'h-0 w-0'
                : 'fixed z-40 h-full w-full bg-black/40 opacity-100',
        ]"
        @click="hideSideNavBar"
    ></div>
    <div
        :class="[topNavBarIsPrimary ? '-translate-x-full' : 'translate-x-0']"
        class="duration-400 fixed left-0 top-0 z-50 h-screen w-60 bg-white shadow-lg transition-transform"
    >
        <h2 class="p-2 text-2xl">Yummy</h2>
        <ul class="p-4">
            <li class="py-2"><a href="#">Option 1</a></li>
            <li class="py-2"><a href="#">Option 2</a></li>
            <li class="py-2"><a href="#">Option 3</a></li>
        </ul>
    </div>
    <nav
        class="fixed h-20 w-screen items-center justify-between bg-gray-300 p-8"
        :class="widthGreaterThanMobile ? 'hidden' : 'flex'"
    >
        <div class="flex items-center gap-4">
            <a href="#" class="flex items-center" @click="toggleSideNavBar">
                <font-awesome-icon :icon="['fas', 'bars']" class="text-2xl" />
            </a>
            <h2 class="text-2xl">Yummy</h2>
        </div>
        <div>
            <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-2xl" />
        </div>
    </nav>
</template>
