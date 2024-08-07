<script setup>
import { yummy_backend } from "declarations/yummy_backend/index";
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { useMessageStore } from "../store/message";

const messageStore = useMessageStore();
const authStore = useAuthStore();
const { whoamiActor } = storeToRefs(authStore);

const title = ref("");
const instructions = ref([]);
const ingredients = ref([]);
const prepTime = ref(0);
const availableCuisines = [
    "American",
    "Brazilian",
    "Asian",
    "British",
    "Chinese",
    "French",
    "Greek",
    "Korean",
    "Mexican",
    "Portuguese",
    "Spanish",
    "Polish",
    "Swedish",
];
const selectedCuisines = ref([]);
const availableTags = [
    "Vegetarian",
    "Vegan",
    "Gluten-Free",
    "Low-Carb",
    "Keto-Friendly",
    "Low-Fat",
    "Soup",
    "Salad",
    "Breakfast",
    "Dinner",
    "Dessert",
    "Snack",
    "High-Protein",
];
const selectedTags = ref([]);
const uploadedImage = ref("");

const addingInstruction = ref(false);
const newInstruction = ref("");
const addingIngredient = ref(false);
const newIngredient = ref("");

function addInstruction() {
    if (newInstruction.value.trim()) {
        instructions.value.push(newInstruction.value.trim());
        newInstruction.value = "";
    }
    addingInstruction.value = false;
}

function cancelAddingInstruction() {
    newInstruction.value = "";
    addingInstruction.value = false;
}

function addIngredient() {
    if (newIngredient.value.trim()) {
        ingredients.value.push(newIngredient.value.trim());
        newIngredient.value = "";
    }
    addingIngredient.value = false;
}

function cancelAddingIngredient() {
    newIngredient.value = "";
    addingIngredient.value = false;
}

function toggleTag(tag) {
    const index = selectedTags.value.indexOf(tag);
    if (index === -1) {
        selectedTags.value.push(tag);
    } else {
        selectedTags.value.splice(index, 1);
    }
}

function toggleCuisine(cuisine) {
    const index = selectedCuisines.value.indexOf(cuisine);
    if (index === -1) {
        selectedCuisines.value.push(cuisine);
    } else {
        selectedCuisines.value.splice(index, 1);
    }
}
function handleImageUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    // check file size
    if (file.size > 1024 * 1024) {
        messageStore.showMessage("File size exceeds limit (1MB)", "warning");
        event.target.value = "";
        return;
    }

    const reader = new FileReader();
    reader.onload = function (e) {
        uploadedImage.value = e.target.result;
    };
    reader.readAsDataURL(file);
}

function createErrorFromObject(response) {
    let error_key = Object.keys(response)[0];
    let msg = response[error_key].msg;
    messageStore.showMessage(msg, "error");
}
async function submitRecipe() {
    messageStore.showMessage("Creating recipe...", "info");
    // check is user authorized to create recipe
    if (!whoamiActor.value) {
        messageStore.showMessage("You are not logged in. Please log in first.", "warning");
        return;
    }
    // validate for empty fields
    if (!instructions.value.length || !ingredients.value.length || !selectedTags.value.length) {
        messageStore.showMessage("Recipe should have at least one instruction, one ingredient, and one tag", "warning");
        return;
    }
    let user_index = await whoamiActor.value.get_user_index();
    if (user_index.Err) {
        messageStore.showMessage(user_index.Err);
        return;
    } else {
        user_index = user_index.Ok;
    }

    let response = await yummy_backend.add_recipe({
        name: title.value,
        instructions: instructions.value,
        ingredients: ingredients.value,
        tags: selectedTags.value,
        cuisines: selectedCuisines.value,
        total_time_in_seconds: prepTime.value * 60,
        image: uploadedImage.value,
        author_id: user_index,
    });
    if (response.Err) {
        createErrorFromObject(response.Err);
        title.value = "";
        return;
    }
    messageStore.hideMessage();

    // // Reset title of form after submission
    title.value = "";
    selectedTags.value = [];
    selectedCuisines.value = [];
    instructions.value = [];
    ingredients.value = [];
    prepTime.value = 0;
    uploadedImage.value = "";
    document.getElementById("image").value = "";
}
</script>

<template>
    <div class="relative">
        <div class="flex min-h-screen w-full items-center justify-center bg-gray-800 p-4 text-gray-100">
            <div class="w-full max-w-3xl rounded-lg bg-gray-700 p-8 shadow-xl">
                <h1 class="mb-8 text-center text-3xl font-bold text-indigo-300">Add New Recipe</h1>
                <form @keydown.enter.prevent @submit.prevent="submitRecipe" class="space-y-6">
                    <div>
                        <label for="title" class="mb-2 block text-lg font-medium text-gray-300">Title</label>
                        <input
                            v-model="title"
                            type="text"
                            id="title"
                            required
                            class="w-full rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                        />
                    </div>

                    <div>
                        <label class="mb-2 block text-lg font-medium text-gray-300">Instructions</label>
                        <ul class="mb-2 space-y-2">
                            <li v-for="(instruction, index) in instructions" :key="index" class="flex items-center">
                                <span class="flex-grow">{{ instruction }}</span>
                                <button
                                    @click="instructions.splice(index, 1)"
                                    class="ml-2 p-1 text-gray-400 hover:text-red-500"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-5 w-5"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </button>
                            </li>
                        </ul>
                        <div v-if="addingInstruction" class="flex items-center">
                            <textarea
                                v-model="newInstruction"
                                @keydown.enter.prevent="addInstruction"
                                maxlength="300"
                                rows="3"
                                class="w-full resize-y rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                            ></textarea>
                            <button @click="addInstruction" class="ml-2 p-1 text-green-500 hover:text-green-400">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-6 w-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M5 13l4 4L19 7"
                                    />
                                </svg>
                            </button>
                            <button @click="cancelAddingInstruction" class="ml-2 p-1 text-red-500 hover:text-red-400">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-6 w-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </div>
                        <button
                            v-else
                            @click="addingInstruction = true"
                            class="mt-2 rounded-md bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50"
                        >
                            Add Instruction
                        </button>
                    </div>

                    <div>
                        <label class="mb-2 block text-lg font-medium text-gray-300">Ingredients</label>
                        <ul class="mb-2 space-y-2">
                            <li v-for="(ingredient, index) in ingredients" :key="index" class="flex items-center">
                                <span class="flex-grow">{{ ingredient }}</span>
                                <button
                                    @click="ingredients.splice(index, 1)"
                                    class="ml-2 p-1 text-gray-400 hover:text-red-500"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-5 w-5"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </button>
                            </li>
                        </ul>
                        <div v-if="addingIngredient" class="flex items-center">
                            <textarea
                                v-model="newIngredient"
                                @keydown.enter.prevent="addIngredient"
                                maxlength="300"
                                rows="3"
                                class="w-full resize-y rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                            ></textarea>
                            <button @click="addIngredient" class="ml-2 p-1 text-green-500 hover:text-green-400">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-6 w-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M5 13l4 4L19 7"
                                    />
                                </svg>
                            </button>
                            <button @click="cancelAddingIngredient" class="ml-2 p-1 text-red-500 hover:text-red-400">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-6 w-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </div>
                        <button
                            v-else
                            @click="addingIngredient = true"
                            class="mt-2 rounded-md bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50"
                        >
                            Add Ingredient
                        </button>
                    </div>

                    <div>
                        <label class="mb-2 block text-lg font-medium text-gray-300">Tags</label>
                        <div class="flex flex-wrap justify-center gap-3">
                            <button
                                v-for="(tag, index) in availableTags"
                                :key="index"
                                @click.prevent="toggleTag(tag)"
                                :class="[
                                    'rounded-full px-4 py-2 text-lg font-medium',
                                    selectedTags.includes(tag)
                                        ? 'bg-indigo-600 text-white'
                                        : 'bg-gray-600 text-gray-300 hover:bg-gray-500',
                                ]"
                            >
                                {{ tag }}
                            </button>
                        </div>
                    </div>

                    <div>
                        <label class="mb-2 block text-lg font-medium text-gray-300">Cuisines</label>
                        <div class="flex flex-wrap justify-center gap-3">
                            <button
                                v-for="(cuisine, index) in availableCuisines"
                                :key="index"
                                @click.prevent="toggleCuisine(cuisine)"
                                :class="[
                                    'rounded-full px-4 py-2 text-lg font-medium',
                                    selectedCuisines.includes(cuisine)
                                        ? 'bg-indigo-600 text-white'
                                        : 'bg-gray-600 text-gray-300 hover:bg-gray-500',
                                ]"
                            >
                                {{ cuisine }}
                            </button>
                        </div>
                    </div>

                    <div>
                        <label for="prepTime" class="mb-2 block text-lg font-medium text-gray-300"
                            >Preparation Time (minutes)</label
                        >
                        <input
                            v-model.number="prepTime"
                            type="number"
                            id="prepTime"
                            min="3"
                            required
                            class="w-full rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                        />
                    </div>

                    <div>
                        <label for="image" class="mb-2 block text-lg font-medium text-gray-300">Upload Images</label>
                        <input
                            type="file"
                            id="image"
                            @change="handleImageUpload"
                            accept="image/*"
                            class="w-full text-lg text-gray-400 file:mr-4 file:rounded-full file:border-0 file:bg-gray-600 file:px-4 file:py-3 file:text-lg file:font-semibold file:text-gray-200 hover:file:bg-gray-500"
                        />
                    </div>

                    <div class="flex items-center justify-center space-x-2">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6 text-gray-400"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4 5a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V7a2 2 0 00-2-2h-1.586a1 1 0 01-.707-.293l-1.121-1.121A2 2 0 0011.172 3H8.828a2 2 0 00-1.414.586L6.293 4.707A1 1 0 015.586 5H4zm6 9a3 3 0 100-6 3 3 0 000 6z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        <span class="text-lg text-gray-400">{{ uploadedImage ? 1 : 0 }} image(s) selected</span>
                    </div>

                    <div class="flex justify-center">
                        <button
                            type="submit"
                            class="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-6 py-3 text-lg font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="mr-2 h-6 w-6"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                            Add Recipe
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>
