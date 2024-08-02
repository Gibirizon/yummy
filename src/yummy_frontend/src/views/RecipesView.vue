<template>
    <div>
        <RecipePage :hero-image="heroImages[currentType]" :page-title="pageTitles[currentType]" :recipes="recipeData" />
    </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from "vue";
import { useRoute } from "vue-router";
import RecipePage from "../components/recipe/RecipePage.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
console.log("RecipesView.vue");

const route = useRoute();

// Compute the current route name or its parent for nested routes
const currentType = computed(() => {
    console.log("route: ", route);
    return route.params.type;
});

// Define hero images for each route
const heroImages = {
    Popular: "/images/popular-hero.jpg",
    Breakfast: "/images/breakfast-hero.jpg",
    Dinner: "/images/dinner-hero.jpg",
    Dessert: "/images/dessert-hero.jpg",
    Snack: "/images/snack-hero.jpg",
    Yours: "/images/yours-hero.jpg",
    Others: "/images/others-hero.jpg",
};

// Define page titles for each route
const pageTitles = {
    Popular: "Popular",
    Breakfast: "Breakfast",
    Dinner: "Dinner",
    Dessert: "Dessert",
    Snack: "Snack",
    Yours: "Your Recipes",
    Others: "Other User's Recipes",
};

watch(currentType, () => {
    console.log("currentType: ", currentType.value);
    getRecipeData();
});

async function getRecipeData() {
    // console.log("getRecipeData: ", type);
    recipeData.value = [];
    const all_data = await yummy_backend.take_recipes_of_specific_type(currentType.value);
    if (all_data.Err) {
        console.log("Error: ", all_data.Err);
        return;
    }
    for (const recipe of all_data.Ok) {
        let imageUrl;
        try {
            const responseImage = await yummy_backend.get_image(recipe.name);
            const imageData = new Uint8Array(responseImage.Bytes);
            const blob = new Blob([imageData], { type: "image/jpeg" });
            imageUrl = URL.createObjectURL(blob);
        } catch (error) {
            console.error("Error fetching image:", error);
        }

        recipeData.value.push({
            name: recipe.name,
            image: imageUrl,
            time: recipe.total_time,
            tags: recipe.tags,
        });
    }
}

// Define or fetch recipe data for each route
const recipeData = ref([]);

onMounted(async () => {
    getRecipeData(currentType.value);
});
</script>
