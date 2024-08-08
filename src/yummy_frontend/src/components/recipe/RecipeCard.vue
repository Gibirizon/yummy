<template>
    <div
        class="relative overflow-hidden rounded-lg bg-gray-700 shadow-lg transition-all duration-300 ease-in-out focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 hover:scale-105 hover:bg-gray-600 hover:shadow-2xl"
        tabindex="0"
        @click="handleCardClick"
    >
        <img :src="recipe.image" :alt="recipe.name" class="h-[200px] w-full object-cover sm:h-[250px] md:h-[300px]" />
        <div class="p-4">
            <h3 class="mb-2 text-[18px] font-semibold text-white sm:text-xl">{{ recipe.name }}</h3>
            <div class="mb-2 flex items-center text-gray-300">
                <Clock class="mr-1 h-4 w-4" />
                <span class="text-sm sm:text-[16px]">{{ recipe.time }} mins</span>
            </div>
            <div v-if="recipe.tags && recipe.tags.length > 0" class="mb-2 flex flex-wrap">
                <span
                    v-for="tag in recipe.tags"
                    :key="tag"
                    class="duration-400 mb-2 mr-2 rounded-full bg-gray-500 px-2 py-1 text-[12px] text-gray-200 transition-colors hover:bg-gray-400 hover:text-gray-800 sm:text-sm"
                >
                    #{{ tag }}
                </span>
            </div>
            <div v-if="recipe.author" class="text-sm text-gray-400">
                <User class="mr-1 inline h-4 w-4" />
                <span>By {{ recipe.author }}</span>
            </div>
        </div>
        <button
            v-if="recipe.yourRecipes"
            @click.stop="handleTrashClick"
            class="absolute bottom-2 right-2 rounded-full bg-red-500 p-2 transition-colors duration-200 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400"
        >
            <Trash2 class="h-5 w-5 text-white" />
        </button>
    </div>
</template>

<script setup>
import { Clock, User, Trash2 } from "lucide-vue-next";

const props = defineProps({
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

const emit = defineEmits(["item-click", "trash-click"]);

const handleCardClick = () => {
    emit("item-click", props.recipe);
};

const handleTrashClick = (event) => {
    event.stopPropagation();
    emit("trash-click", props.recipe);
};
</script>
