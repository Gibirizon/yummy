<template>
    <DeleteConfirm v-if="showDeleteConfirmation" @close="showDeleteConfirmation = false" @confirm="confirmDel" />
    <div>
        <RecipePage
            :hero-image="ImagesAndPageTitles[currentType].image"
            :page-title="ImagesAndPageTitles[currentType].title"
            :recipes="recipesData"
            @delete-recipe="deleteRecipe"
        />
    </div>
</template>

<script setup>
import { useDelete } from "../composables/delete";
import DeleteConfirm from "../components/DeleteConfirm.vue";
import { ref, onBeforeMount, computed, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import RecipePage from "../components/recipe/RecipePage.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useAuthStore } from "../store/auth";
import { retryICCall } from "../retry/icRetry";
import { useMessageStore } from "../store/message";

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const { isAuthenticated, whoamiActor } = storeToRefs(authStore);
const recipesData = ref([]);
const messageStore = useMessageStore();
const recipeNameToDelete = ref("");
const { retryCall } = retryICCall();
const { showDeleteConfirmation, confirmDelete } = useDelete();

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
        title: "All Users' Recipes",
    },
};

watch(currentType, async () => {
    console.log("currentType: ", currentType.value);
    showDeleteConfirmation.value = false;
    await getRecipeData();
});

async function getRecipeData() {
    // clean recipeData
    recipesData.value = [];
    let all_recipes = [];

    // taking correct recipes from backend
    if (currentType.value === "yours") {
        if (!isAuthenticated.value) {
            router.push({ name: "home", query: { canisterId: route.query.canisterId } });
            return;
        }
        let user_index = await retryCall(() => whoamiActor.value.get_user_index());
        if (user_index.Err) {
            messageStore.showMessage(user_index.Err.UserNotFound.msg, "error");
            return;
        }
        all_recipes = await yummy_backend.take_recipes_by_author(user_index.Ok);
        console.log("user recipes: ", all_recipes);
    } else if (currentType.value === "users") {
        all_recipes = await yummy_backend.take_user_recipes_with_author_names();
    } else {
        all_recipes = await yummy_backend.take_recipes_of_specific_type(
            currentType.value.charAt(0).toUpperCase() + currentType.value.slice(1)
        );
    }

    for (const recipe of all_recipes) {
        recipesData.value.push({
            name: recipe.name,
            image: "",
            time: recipe.total_time,
            tags: recipe.tags,
            author: recipe.author.length > 0 ? recipe.author[0] : null,
            yourRecipes: currentType.value === "yours" ? true : false,
        });
    }

    // getting images for recipes
    for (const recipe of recipesData.value) {
        let imageUrl;
        if (recipe.author) {
            // recipe created by user
            const responseImage = await yummy_backend.get_image(recipe.name);
            if (!responseImage.length) {
                // no image found, display default
                imageUrl = "/images/default-recipe.jpg";
            } else {
                imageUrl = responseImage;
            }
        } else {
            // recipe added from data
            const cleanedName = recipe.name.replace(/[ ,&']/g, "");
            imageUrl = `/images/recipes_data/${cleanedName}.jpg`;
        }
        recipe.image = imageUrl;
    }
}

async function confirmDel() {
    await confirmDelete(() => yummy_backend.delete_recipe(recipeNameToDelete.value));
    recipeNameToDelete.value = "";
    await getRecipeData(currentType.value);
}

async function deleteRecipe(name) {
    showDeleteConfirmation.value = true;
    recipeNameToDelete.value = name;
}

onBeforeMount(async () => {
    setTimeout(async () => {
        await getRecipeData();
    }, 100);
});
</script>
