<template>
    <div>
        <RecipePage
            :hero-image="ImagesAndPageTitles[currentType].image"
            :page-title="ImagesAndPageTitles[currentType].title"
            :recipes="recipesData"
        />
    </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from "vue";
import { useRoute } from "vue-router";
import RecipePage from "../components/recipe/RecipePage.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useAuthStore } from "../store/auth";
console.log("RecipesView.vue");

const route = useRoute();

// Compute the current route name or its parent for nested routes
const currentType = computed(() => {
    return route.params.type;
});

// Define hero images for each route
const ImagesAndPageTitles = {
    popular: {
        image: "/images/popular-hero.jpg",
        title: "Popular",
    },
    breakfast: {
        image: "/images/breakfast-hero.jpg",
        title: "Breakfast",
    },
    dinner: {
        image: "/images/dinner-hero.jpg",
        title: "Dinner",
    },
    dessert: {
        image: "/images/dessert-hero.jpg",
        title: "Dessert",
    },
    snack: {
        image: "/images/snack-hero.jpg",
        title: "Snack",
    },
    yours: {
        image: "/images/yours-hero.jpg",
        title: "Your Recipes",
    },
    users: {
        image: "/images/users-hero.jpg",
        title: "All User's Recipes",
    },
};

watch(currentType, () => {
    console.log("currentType: ", currentType.value);
    getRecipeData();
});

const authStore = useAuthStore();
console.log("authstore is ready in recipesview: ", authStore.isReady);
async function getRecipeData() {
    // clean recipeData
    recipesData.value = [];
    let all_recipes = [];

    // taking correct recipes from backend
    if (currentType.value === "yours") {
        let user = await authStore.whoamiActor?.get_user_info();
        if (user.Err) {
            console.log("Error: ", user.Err);
            return;
        }
        all_recipes = await yummy_backend.take_user_recipes(user.Ok);
    } else if (currentType.value === "users") {
        all_recipes = await yummy_backend.take_all_users_recipes();
        all_recipes = all_recipes.flat();
    } else {
        all_recipes = await yummy_backend.take_recipes_of_specific_type(
            currentType.value.charAt(0).toUpperCase() + currentType.value.slice(1)
        );
    }

    // getting images for recipes
    for (const recipe of all_recipes) {
        let imageUrl;
        try {
            const responseImage = await yummy_backend.get_image(recipe.name);
            const responseKey = Object.keys(responseImage)[0];
            // image added by user from url
            if (responseKey == "Url") {
                imageUrl = responseImage.Url;
            } else if (responseImage.Bytes.length == 0) {
                // no image found, display default
                imageUrl = "/images/default-recipe.jpg";
            } else {
                const imageData = new Uint8Array(responseImage.Bytes);
                const blob = new Blob([imageData], { type: "image/jpeg" });
                imageUrl = URL.createObjectURL(blob);
            }
        } catch (error) {
            console.error("Error fetching image:", error);
        }

        recipesData.value.push({
            name: recipe.name,
            image: imageUrl,
            time: recipe.total_time,
            tags: recipe.tags,
            author: recipe.author.length > 0 ? recipe.author[0] : null,
        });
    }
}

// Define or fetch recipe data for each route
const recipesData = ref([]);

onMounted(async () => {
    getRecipeData(currentType.value);
});
</script>
