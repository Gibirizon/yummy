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
const tagRecipes = ref([]);

async function getPopularRecipes() {
    if (popularRecipes.value.length > 0) return;
    console.log("Hi");
    await yummy_backend.get_popular_recipes_len().then(async (length) => {
        console.log(length);
        for (let i = 0; i < length; i++) {
            const RecipeData = await yummy_backend.get_popular_recipe(i);
            let new_recipe = await createRecipe(RecipeData);
            popularRecipes.value.push(new_recipe);
        }
    });
}

async function getTagRecipes() {
    if (tagRecipes.value.length > 0) return;
    await yummy_backend.get_tag_recipes_len().then(async (length) => {
        console.log(length);
        for (let i = 0; i < length; i++) {
            const RecipeData = await yummy_backend.get_tag_recipe(i);
            let new_recipe = await createRecipe(RecipeData);
            tagRecipes.value.push(new_recipe);
        }
    });
}

async function createRecipe(data) {
    const imageData = data.main_image;

    // Convert Uint8Array to string in chunks
    const chunkSize = 8192;
    let binary = "";
    for (let i = 0; i < imageData.length; i += chunkSize) {
        binary += String.fromCharCode.apply(null, imageData.subarray(i, i + chunkSize));
    }

    const imageBlob = btoa(binary);
    let imageInfo = `data:image/jpeg;base64,${imageBlob}`;
    let new_recipe = new RecipeBrief(data.name, imageInfo, data.total_time_in_seconds / 60);
    return new_recipe;
}
function handleItemClick(item) {
    console.log("Clicked item:", item);
}

onMounted(() => {
    getPopularRecipes();
    getTagRecipes();
});

onUnmounted(() => {
    // popularRecipes.value = [];
    // tagRecipes.value = [];
});
</script>

<template>
    <h2 class="p-4 text-center text-4xl text-white">Popular Recipes</h2>
    <SingleList v-if="popularRecipes" :recipes="popularRecipes" @item-click="handleItemClick" />
    <h2 class="p-4 text-center text-4xl text-white">Breakfast</h2>
    <SingleList v-if="tagRecipes" :recipes="tagRecipes" @item-click="handleItemClick" />
    <h2 class="p-4 text-center text-4xl text-white">Breakfast</h2>
    <SingleList v-if="tagRecipes" :recipes="tagRecipes" @item-click="handleItemClick" />
</template>
