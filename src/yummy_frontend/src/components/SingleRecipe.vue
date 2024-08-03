<script setup>
import { ref, computed, onBeforeMount } from "vue";
import Message from "./Message.vue";
import { useRouter, useRoute } from "vue-router";
import { Clock, ChefHat, Tag, ChevronDown, ChevronUp, Utensils, List } from "lucide-vue-next";
import { yummy_backend } from "declarations/yummy_backend/index";

const route = useRoute();
const router = useRouter();

async function getRecipe() {
    const recipeName = decodeURIComponent(route.params.name);
    let recipeInfo;

    // trying to get recipe from downloaded info from api
    const recipeFromAPI = await yummy_backend.take_recipe(recipeName);
    if (recipeFromAPI.Ok) {
        recipeInfo = recipeFromAPI.Ok;
    } else {
        // getting image created by users
        let recipeFromUsers = await yummy_backend.take_recipe_by_name(recipeName);

        // no recipe found
        if (recipeFromUsers.Err) {
            console.log(recipeFromUsers.Err);
            createMessage(recipeFromUsers.Err.RecipeNotFound.msg, "error");
            return;
        }

        recipeInfo = recipeFromUsers.Ok;
    }
    let imageUrl;
    try {
        const responseImage = await yummy_backend.get_image(recipeName);
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

    // store informations in variable
    recipe.value = {
        name: recipeName,
        image: imageUrl,
        instructions: recipeInfo.instructions,
        ingredients: recipeInfo.ingredients,
        tags: recipeInfo.tags,
        prepTime: recipeInfo.total_time_in_seconds / 60,
        cuisines: recipeInfo.cuisines[0] ? recipeInfo.cuisines[0] : [],
    };
}

const recipe = ref({
    name: "",
    image: "",
    instructions: [],
    ingredients: [],
    tags: [],
    prepTime: 0,
    cuisines: [],
});

const showFullInstructions = ref(false);
const showFullIngredients = ref(false);

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

const showMessage = ref(false);
const messageText = ref("");
const messageType = ref("");
function createMessage(msg, type) {
    messageText.value = msg;
    messageType.value = type;
    showMessage.value = true;
}
function closeMessage() {
    showMessage.value = false;
}

onBeforeMount(async () => {
    await getRecipe();
});
</script>

<template>
    <Transition name="slide">
        <Message v-if="showMessage" :text="messageText" :type="messageType" @close="closeMessage" />
    </Transition>
    <div class="min-h-screen w-full bg-gray-800 p-6 text-gray-100 md:p-12">
        <div class="mx-auto max-w-4xl overflow-hidden rounded-3xl bg-gray-700 shadow-xl">
            <img :src="recipe.image" :alt="recipe.name" class="h-[400px] w-full object-cover" />
            <div class="p-8 md:p-10">
                <h1 class="mb-6 text-4xl font-bold text-indigo-300 md:text-5xl" v-if="recipe.name.length > 0">
                    {{ recipe.name }}
                </h1>

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
