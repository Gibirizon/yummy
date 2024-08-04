<template>
    <Transition name="slide">
        <Message v-if="showMessage" :text="messageText" :type="messageType" @close="closeMessage" />
    </Transition>
    <DeleteConfirm v-if="showDeleteConfirmation" @close="closeDeleteConfirmation" @confirm="confirmDelete" />
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
import DeleteConfirm from "../components/DeleteConfirm.vue";
import { ref, onMounted, computed, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import RecipePage from "../components/recipe/RecipePage.vue";
import Message from "../components/Message.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useAuthStore } from "../store/auth";
import { retryICCall } from "../retry/icRetry";

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const { isAuthenticated, whoamiActor } = storeToRefs(authStore);
const recipesData = ref([]);
const showMessage = ref(false);
const messageText = ref("");
const messageType = ref("");
const showDeleteConfirmation = ref(false);
const recipeNameToDelete = ref("");

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
    await getRecipeData();
});

watch(isAuthenticated, async () => {
    if (currentType.value === "yours") {
        await getRecipeData();
    }
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
        all_recipes = await retryICCall(() => whoamiActor.value.take_user_recipes());
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
            yourRecipes: currentType.value === "yours" ? true : false,
        });
    }
}

function closeDeleteConfirmation() {
    showDeleteConfirmation.value = false;
}

async function confirmDelete() {
    createMessage("Deleting recipe...", "warning");
    closeDeleteConfirmation();
    if (!isAuthenticated.value) {
        createMessage("Not logged in", "error");
        return;
    }
    const indexResponse = await whoamiActor.value.get_user_index();
    if (indexResponse.Err) {
        createMessage(indexResponse.Err.UserNotFound.msg, "warning");
        return;
    }

    let deleteResponse = await yummy_backend.delete_recipe(recipeNameToDelete.value, indexResponse.Ok);
    if (deleteResponse.Err) {
        createMessage(deleteResponse.Err.RecipeNotFound.msg, "warning");
        return;
    }

    createMessage(deleteResponse.Ok, "success");
    recipeNameToDelete.value = "";
    await getRecipeData(currentType.value);
}

async function deleteRecipe(name) {
    showDeleteConfirmation.value = true;
    recipeNameToDelete.value = name;
}

function createMessage(msg, type) {
    messageText.value = msg;
    messageType.value = type;
    showMessage.value = true;
}

function closeMessage() {
    showMessage.value = false;
}

onMounted(async () => {
    setTimeout(async () => {
        await getRecipeData();
    }, 100);
});
</script>
