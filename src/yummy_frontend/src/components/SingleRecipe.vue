// TODO - add delete button
<script setup>
import { ref, computed, onBeforeMount, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Trash2, User, Clock, ChefHat, Tag, ChevronDown, ChevronUp, Utensils, List } from "lucide-vue-next";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useAuthStore } from "../store/auth";
import { storeToRefs } from "pinia";
import { useMessageStore } from "../store/message";
import DeleteConfirm from "./DeleteConfirm.vue";
import { useDelete } from "../composables/delete";

const { showDeleteConfirmation, confirmDelete } = useDelete();
const authStore = useAuthStore();
const { isAuthenticated, whoamiActor } = storeToRefs(authStore);
const recipe = ref(null);
const isRecipeOwner = ref(false);
const showFullInstructions = ref(false);
const showFullIngredients = ref(false);
const messageStore = useMessageStore();
const route = useRoute();
const router = useRouter();
watch(
    () => route.params,
    async () => {
        await getRecipe();
    }
);

watch(isAuthenticated, async () => {
    console.log("isAuthenticated: ", isAuthenticated.value);
    if (isAuthenticated.value && recipe.value && recipe.value.author_id) {
        console.log("Checking if recipe belongs to user");
        await checkDoesRecipeBelongsToUser(recipe.value.author_id);
    }
});

async function getRecipe() {
    const recipeName = decodeURIComponent(route.params.name);
    let recipeInfo;

    const recipeResponse = await yummy_backend.take_recipe(recipeName);

    // no recipe found
    if (recipeResponse.Err) {
        console.log(recipeResponse.Err);
        messageStore.showMessage(recipeResponse.Err.RecipeNotFound.msg, "error");
        return;
    }

    console.log("Recipe: ", recipeResponse.Ok);
    recipeInfo = recipeResponse.Ok[0];

    // store info wihtout image
    recipe.value = {
        name: recipeName,
        image: "",
        instructions: recipeInfo.instructions,
        ingredients: recipeInfo.ingredients,
        tags: recipeInfo.tags,
        prepTime: recipeInfo.total_time_in_seconds / 60,
        cuisines: recipeInfo.cuisines,
        author: recipeResponse.Ok[1].length ? recipeResponse.Ok[1][0] : "",
        author_id: recipeInfo.author_id.length ? recipeInfo.author_id[0] : 0,
    };

    let imageUrl;
    if (recipeInfo.author_id.length) {
        // recipe created by user
        const responseImage = await yummy_backend.get_image(recipeName);

        if (!responseImage.length) {
            // no image found, display default
            imageUrl = "/images/default-recipe.jpg";
        } else {
            imageUrl = responseImage;
        }
    } else {
        // recipe added from data
        const cleanedName = recipeName.replace(/[ ,&']/g, "");
        imageUrl = `/images/recipes_data/${cleanedName}.jpg`;
    }

    // add image  to recipe
    recipe.value.image = imageUrl;

    if (recipeInfo.author_id.length) {
        checkDoesRecipeBelongsToUser(recipeInfo.author_id[0]);
    }
}

async function checkDoesRecipeBelongsToUser(author_id) {
    if (!isAuthenticated.value) {
        return;
    }
    const responseIndex = await whoamiActor.value.get_user_index();
    if (responseIndex.Err) {
        return;
    }
    if (responseIndex.Ok === author_id) {
        isRecipeOwner.value = true;
    }
}

async function confirmDel() {
    await confirmDelete(() => yummy_backend.delete_recipe(recipe.value.name));
    router.push({ name: "home", query: { canisterId: route.query.canisterId } });
}
const toggleInstructions = () => {
    showFullInstructions.value = !showFullInstructions.value;
};

const toggleIngredients = () => {
    showFullIngredients.value = !showFullIngredients.value;
};

const displayedInstructions = computed(() =>
    showFullInstructions.value ? recipe.value.instructions : recipe.value.instructions.slice(0, 3)
);

const displayedIngredients = computed(() =>
    showFullIngredients.value ? recipe.value.ingredients : recipe.value.ingredients.slice(0, 3)
);

onBeforeMount(async () => {
    await getRecipe();
});
</script>

<template>
    <DeleteConfirm v-if="showDeleteConfirmation" @close="showDeleteConfirmation = false" @confirm="confirmDel" />
    <div v-if="recipe" class="min-h-screen w-full bg-gray-800 p-6 text-gray-100 md:p-12">
        <div class="relative mx-auto max-w-4xl overflow-hidden rounded-3xl bg-gray-700 shadow-xl">
            <button
                v-if="isRecipeOwner"
                @click.stop="showDeleteConfirmation = true"
                class="absolute right-4 top-5 rounded-full bg-red-500 p-2 transition-colors duration-200 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400"
            >
                <Trash2 class="h-6 w-6 text-white" />
            </button>
            <img :src="recipe.image" :alt="recipe.name" class="h-[400px] w-full object-cover" />
            <div class="p-8 md:p-10">
                <h1 class="mb-6 text-4xl font-bold text-indigo-300 md:text-5xl" v-if="recipe.name.length > 0">
                    {{ recipe.name }}
                </h1>
                <div class="mb-8 flex flex-wrap items-center gap-6" v-if="recipe.author !== ''">
                    <div class="flex items-center">
                        <User class="mr-3 h-7 w-7 text-indigo-400" />
                        <span class="text-xl">Created by {{ recipe.author }}</span>
                    </div>
                </div>

                <div class="mb-8 flex flex-wrap items-center gap-6" v-if="recipe.prepTime > 0">
                    <div class="flex items-center">
                        <Clock class="mr-3 h-7 w-7 text-indigo-400" />
                        <span class="text-xl">{{ recipe.prepTime }} mins</span>
                    </div>
                </div>

                <div class="mb-10">
                    <h2 class="mb-4 flex items-center text-2xl font-semibold sm:text-3xl">
                        <Utensils class="mr-3 h-8 w-8 text-indigo-400" />
                        Ingredients
                    </h2>
                    <ul class="list-inside list-disc space-y-2 text-lg">
                        <li v-for="(ingredient, index) in displayedIngredients" :key="index">
                            {{ ingredient }}
                        </li>
                    </ul>
                    <button
                        v-if="recipe.ingredients.length > 3"
                        @click="toggleIngredients"
                        class="mt-4 flex items-center text-lg text-indigo-400 hover:text-indigo-300"
                    >
                        <component :is="showFullIngredients ? ChevronUp : ChevronDown" class="mr-2 h-5 w-5" />
                        {{ showFullIngredients ? "Show Less" : `Show All (${recipe.ingredients.length})` }}
                    </button>
                </div>

                <div class="mb-10">
                    <h2 class="mb-4 flex items-center text-2xl font-semibold sm:text-3xl">
                        <List class="mr-3 h-8 w-8 text-indigo-400" />
                        Instructions
                    </h2>
                    <ol class="list-inside list-decimal space-y-3">
                        <li v-for="(instruction, index) in displayedInstructions" :key="index" class="pl-2 text-lg">
                            {{ instruction }}
                        </li>
                    </ol>
                    <button
                        v-if="recipe.instructions.length > 3"
                        @click="toggleInstructions"
                        class="mt-4 flex items-center text-lg text-indigo-400 hover:text-indigo-300"
                    >
                        <component :is="showFullInstructions ? ChevronUp : ChevronDown" class="mr-2 h-5 w-5" />
                        {{ showFullInstructions ? "Show Less" : `Show All Steps (${recipe.instructions.length})` }}
                    </button>
                </div>

                <div v-if="recipe.cuisines.length > 0" class="mb-10">
                    <h2 class="mb-4 flex items-center text-2xl font-semibold sm:text-3xl">
                        <ChefHat class="mr-3 h-8 w-8 text-indigo-400" />
                        Cuisines
                    </h2>
                    <div class="flex flex-wrap gap-3">
                        <span
                            v-for="(cuisine, index) in recipe.cuisines"
                            :key="index"
                            class="rounded-full bg-indigo-600 px-4 py-2 text-lg text-white"
                        >
                            {{ cuisine }}
                        </span>
                    </div>
                </div>

                <div v-if="recipe.tags.length > 0">
                    <h2 class="mb-4 flex items-center text-2xl font-semibold sm:text-3xl">
                        <Tag class="mr-3 h-8 w-8 text-indigo-400" />
                        Tags
                    </h2>
                    <div class="flex flex-wrap gap-3">
                        <span
                            v-for="(tag, index) in recipe.tags"
                            :key="index"
                            class="rounded-full bg-indigo-600 px-4 py-2 text-lg text-white"
                        >
                            #{{ tag }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
