<script setup>
import RecipeCard from "./RecipeCard.vue";

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
</script>

<template>
    <div class="min-h-screen bg-gray-800 text-white">
        <!-- Hero Section -->
        <div class="relative mb-8 h-96">
            <img :src="heroImage" alt="Hero Image" class="h-full w-full object-cover" />
            <div
                class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-60 font-['Dancing_Script']"
            >
                <h1 class="text-center text-6xl font-bold md:text-8xl">{{ pageTitle }}</h1>
            </div>
        </div>

        <!-- Recipe Grid -->
        <div class="container mx-auto px-4">
            <div class="grid grid-cols-1 gap-6 md:grid-cols-2 xl:grid-cols-3">
                <RecipeCard v-for="(recipe, index) in recipes" :key="index" :recipe="recipe" />
            </div>
        </div>
    </div>
</template>
