<template>
    <div
        class="overflow-hidden rounded-lg bg-gray-700 shadow-lg transition-all duration-300 ease-in-out focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 hover:scale-105 hover:bg-gray-600 hover:shadow-2xl"
        tabindex="0"
    >
        <img :src="recipe.image" :alt="recipe.name" class="h-[300px] w-full object-cover" />
        <div class="p-4">
            <h3 class="mb-2 text-xl font-semibold text-white">{{ recipe.name }}</h3>
            <div class="mb-2 flex items-center text-gray-300">
                <Clock class="mr-1 h-4 w-4" />
                <span>{{ recipe.time }} mins</span>
            </div>
            <div v-if="recipe.tags && recipe.tags.length > 0" class="mb-2 flex flex-wrap">
                <span
                    v-for="tag in recipe.tags"
                    :key="tag"
                    class="duration-400 mb-2 mr-2 rounded-full bg-gray-500 px-2 py-1 text-sm text-gray-200 transition-colors hover:bg-gray-400 hover:text-gray-800"
                >
                    #{{ tag }}
                </span>
            </div>
            <div v-if="recipe.author" class="text-sm text-gray-400">
                <User class="mr-1 inline h-4 w-4" />
                <span>By {{ recipe.author }}</span>
            </div>
        </div>
    </div>
</template>

<script setup>
import { Clock, User } from "lucide-vue-next";

defineProps({
    recipe: {
        type: Object,
        required: true,
        validator: (value) => {
            if (value.author && typeof value.author !== "string") return false;
            if (value.tags && !Array.isArray(value.tags)) return false;
            return true;
        },
    },
});
</script>
