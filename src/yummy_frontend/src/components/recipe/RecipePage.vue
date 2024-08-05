<script setup>
import { useRouter, useRoute } from "vue-router";
import RecipeCard from "./RecipeCard.vue";

const route = useRoute();
const router = useRouter();
const emit = defineEmits(["delete-recipe"]);

defineProps({
    heroImage: {
        type: String,
        required: true,
    },
    pageTitle: {
        type: String,
        required: true,
    },
    recipes: {
        type: Array,
        required: true,
    },
});

async function handleTrashClick(item) {
    emit("delete-recipe", item.name);
}
function handleItemClick(item) {
    router.push({
        name: "single-recipe",
        query: { canisterId: route.query.canisterId },
        params: { name: encodeURIComponent(item.name) },
    });
}
</script>

<template>
    <div class="min-h-screen bg-gray-800 text-white">
        <!-- Hero Section -->
        <div class="relative mb-8 h-96">
            <img :src="heroImage" alt="Hero Image" class="h-full w-full object-cover" />
            <div class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-60 font-['Courgette']">
                <h1 class="text-center text-6xl font-bold md:text-8xl">{{ pageTitle }}</h1>
            </div>
        </div>

        <!-- Recipe Grid -->
        <div class="container mx-auto mb-10 px-6">
            <div class="grid grid-cols-1 gap-[30px] md:grid-cols-2 xl:grid-cols-3">
                <RecipeCard
                    v-for="(recipe, index) in recipes"
                    :key="index"
                    :recipe="recipe"
                    @item-click="handleItemClick"
                    @trash-click="handleTrashClick"
                />
            </div>
        </div>
    </div>
</template>
