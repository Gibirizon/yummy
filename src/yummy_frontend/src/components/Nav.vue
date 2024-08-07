<script setup>
import { Menu, Search } from "lucide-vue-next";

const emit = defineEmits(["show-menu", "toggle-search"]);

function showMenu() {
    emit("show-menu");
}

function toggleSearch() {
    emit("toggle-search");
}
</script>

<template>
    <nav class="fixed left-0 top-0 z-[110] w-full bg-[#1b263e] text-gray-100 shadow-md transition-all duration-300">
        <div class="relative z-[100] flex h-20 w-full items-center justify-between px-3 sm:px-6 lg:px-8">
            <div class="flex items-center gap-1 sm:gap-4">
                <button
                    @click="showMenu"
                    class="rounded-full p-2 transition-colors duration-200 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-600"
                >
                    <Menu class="h-6 w-6 text-gray-300 sm:h-7 sm:w-7" />
                </button>
                <h2 class="font-['Dancing_Script'] text-3xl font-bold tracking-tight sm:text-5xl">Yummy</h2>
            </div>
            <button
                @click="toggleSearch"
                class="flex items-center gap-2 rounded-md bg-gray-700 px-3 py-2 transition-colors duration-200 hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
            >
                <Search class="h-5 w-5 text-gray-300 sm:h-6 sm:w-6" />
                <span class="text-sm font-medium sm:text-xl">Search</span>
                <span class="ml-1 rounded border border-gray-500 px-1 text-xs text-gray-400 sm:text-sm">Ctrl + K</span>
            </button>
        </div>
    </nav>
</template>
