<script setup>
import { ref, computed, onBeforeMount, watch, onMounted, watchEffect } from "vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useRoute, useRouter } from "vue-router";
import { Search, X, ChevronRight } from "lucide-vue-next";

const props = defineProps({
    isVisible: {
        type: Boolean,
        required: true,
    },
});
watch(
    () => props.isVisible,
    async () => {
        if (props.isVisible) {
            await takeRecipesNames();
        }
    }
);

const emit = defineEmits(["close"]);

const router = useRouter();
const route = useRoute();

const searchInput = ref(null);
const searchQuery = ref("");
const selectedIndex = ref(-1);

// Example recipes list - replace with your actual data
const allRecipesNames = ref([]);

const filteredRecipes = computed(function () {
    if (!searchQuery.value) return [];
    const lowercaseQuery = searchQuery.value.toLowerCase();
    return allRecipesNames.value.filter((name) => {
        return name.toLowerCase().includes(lowercaseQuery);
    });
});

async function takeRecipesNames() {
    allRecipesNames.value = await yummy_backend.take_recipes_names();
}

function closeSearch() {
    emit("close");
    searchQuery.value = "";
    selectedIndex.value = -1;
}

function handleKeydown(event) {
    if (event.key === "ArrowDown") {
        event.preventDefault();
        selectedIndex.value = (selectedIndex.value + 1) % filteredRecipes.value.length;
    } else if (event.key === "ArrowUp") {
        event.preventDefault();
        selectedIndex.value = (selectedIndex.value - 1 + filteredRecipes.value.length) % filteredRecipes.value.length;
    } else if (event.key === "Enter" && selectedIndex.value !== -1) {
        const selectedRecipe = filteredRecipes.value[selectedIndex.value];
        selectRecipe(selectedRecipe);
    }
}

function selectRecipe(name) {
    router.push({
        name: "single-recipe",
        query: { canisterId: route.query.canisterId },
        params: { name: encodeURIComponent(name) },
    });
    closeSearch();
}

onBeforeMount(async () => {
    await takeRecipesNames();
});

watchEffect(() => {
    if (searchInput.value) {
        searchInput.value.focus();
    }
});
</script>

<template>
    <div
        v-if="isVisible"
        class="fixed inset-0 z-[300] flex items-start justify-center p-4 sm:p-0"
        @keydown="handleKeydown"
    >
        <div class="absolute inset-0 bg-black bg-opacity-50 backdrop-blur-sm" @click="closeSearch"></div>
        <div class="relative w-full overflow-hidden rounded-2xl bg-gray-900 shadow-xl sm:mt-[100px] sm:w-[560px]">
            <div class="p-6">
                <button
                    @click="closeSearch"
                    class="absolute right-4 top-2 rounded-full p-2 text-white transition-colors duration-200 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-600"
                >
                    <X class="h-5 w-5" />
                </button>
                <h3 class="mb-4 text-2xl font-semibold text-gray-200">Search for recipes</h3>
                <div class="relative">
                    <Search class="absolute left-4 top-1/2 h-6 w-6 -translate-y-1/2 transform text-gray-400" />
                    <input
                        ref="searchInput"
                        v-model="searchQuery"
                        type="text"
                        placeholder="Search..."
                        class="w-full rounded-lg bg-gray-800 py-3 pl-12 pr-4 text-lg text-white placeholder-gray-400 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        autofocus
                    />
                </div>
                <ul v-if="filteredRecipes.length > 0" class="mt-6 max-h-[60vh] space-y-4 overflow-y-auto">
                    <li
                        v-for="(name, index) in filteredRecipes"
                        :key="index"
                        @click="selectRecipe(name)"
                        class="block rounded-xl bg-gray-800 px-6 py-4 text-lg text-gray-200 transition-colors duration-200 hover:bg-indigo-700 focus:bg-indigo-700 focus:outline-none"
                        :class="{ 'bg-indigo-700': index === selectedIndex }"
                    >
                        <div class="flex items-center justify-between">
                            <span>{{ name }}</span>
                            <ChevronRight v-if="index === selectedIndex" class="h-5 w-5" />
                        </div>
                    </li>
                </ul>
                <div v-else-if="searchQuery" class="mt-4 text-center text-gray-400">No recipes found</div>
            </div>
        </div>
    </div>
</template>
