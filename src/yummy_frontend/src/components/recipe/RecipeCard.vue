<template>
    <div class="overflow-hidden rounded-lg bg-gray-700 shadow-lg transition-shadow duration-300 hover:shadow-xl">
        <img :src="recipe.image" :alt="recipe.title" class="h-60 w-full object-cover" />
        <div class="p-4">
            <h3 class="mb-2 text-xl font-semibold text-white">{{ recipe.name }}</h3>
            <div class="mb-2 flex items-center text-gray-300">
                <Clock class="mr-1 h-4 w-4" />
                <span>{{ recipe.time }} mins</span>
            </div>
            <div class="flex flex-wrap">
                <span
                    v-for="tag in recipe.tags"
                    :key="tag"
                    class="mb-2 mr-2 rounded-full bg-gray-600 px-2 py-1 text-sm text-gray-300"
                >
                    #{{ tag }}
                </span>
            </div>
        </div>
    </div>
</template>

<script setup>
import { Clock } from "lucide-vue-next";

defineProps({
    recipe: {
        type: Object,
        required: true,
    },
});
</script>
