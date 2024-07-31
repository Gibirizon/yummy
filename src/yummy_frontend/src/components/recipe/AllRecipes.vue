<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import SingleList from "./SingleList.vue";
class RecipeBrief {
    constructor(name, mainImage, totalTime = null) {
        this.name = name;
        this.mainImage = mainImage;
        this.totalTime = totalTime;
    }
}

const popularRecipes = ref([]);
const breakfastRecipes = ref([]);
const dinnerRecipes = ref([]);

async function getRecipes() {
    if (popularRecipes.value.length > 0) return;
    const names = await yummy_backend.get_recipes_names();
    if (names.Err) {
        console.log("Error: ", names.Err);
        return;
    }
    for (const name of names.Ok) {
        // get recipe information
        let recipeData = await yummy_backend.get_recipe(name);
        if (recipeData.Err) {
            console.log("Error: ", recipeData.Err);
            continue;
        }
        recipeData = recipeData.Ok;

        // get recipe image
        const imageData = await yummy_backend.get_image(name);

        let new_recipe = await createRecipe(name, recipeData.total_time_in_seconds, imageData.Bytes);
        if (recipeData.popular) {
            popularRecipes.value.push(new_recipe);
        } else {
            if (recipeData.tags.includes("Breakfast")) {
                breakfastRecipes.value.push(new_recipe);
            } else {
                dinnerRecipes.value.push(new_recipe);
            }
        }
    }
}

async function createRecipe(name, total_time_in_seconds, imageData) {
    // Convert Uint8Array to string in chunks
    const chunkSize = 8192;
    let binary = "";
    for (let i = 0; i < imageData.length; i += chunkSize) {
        binary += String.fromCharCode.apply(null, imageData.subarray(i, i + chunkSize));
    }

    const imageBlob = btoa(binary);
    let imageInfo = `data:image/jpeg;base64,${imageBlob}`;
    let new_recipe = new RecipeBrief(name, imageInfo, total_time_in_seconds / 60);
    return new_recipe;
}
function handleItemClick(item) {
    console.log("Clicked item:", item);
}

getRecipes();
</script>

<template>
    <div class="min-h-screen bg-gradient-to-r from-gray-900 via-gray-800 to-gray-700 py-8">
        <h1 class="mb-12 text-center text-5xl font-bold text-white">Our Recipes</h1>

        <section class="mb-16">
            <div class="mb-6 flex items-center justify-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="mr-2 h-8 w-8 text-yellow-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"
                    />
                </svg>
                <h2 class="shadow-text text-3xl font-semibold text-white">Popular Recipes</h2>
            </div>
            <SingleList v-if="popularRecipes" :recipes="popularRecipes" @item-click="handleItemClick" />
        </section>

        <section class="mb-16">
            <div class="mb-6 flex items-center justify-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="mr-2 h-8 w-8 text-orange-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                </svg>
                <h2 class="shadow-text text-3xl font-semibold text-white">Breakfast</h2>
            </div>
            <SingleList v-if="breakfastRecipes" :recipes="breakfastRecipes" @item-click="handleItemClick" />
        </section>

        <section class="mb-16">
            <div class="mb-6 flex items-center justify-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="mr-2 h-8 w-8 text-indigo-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
                <h2 class="shadow-text text-3xl font-semibold text-white">Dinner</h2>
            </div>
            <SingleList v-if="dinnerRecipes" :recipes="dinnerRecipes" @item-click="handleItemClick" />
        </section>
    </div>
</template>

<style scoped>
.shadow-text {
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
}
</style>
