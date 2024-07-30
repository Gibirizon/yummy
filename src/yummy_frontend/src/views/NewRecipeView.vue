<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useAuthStore } from "./../store/auth";

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();

const title = ref("");
const description = ref("");
const ingredients = ref("");
const prepTime = ref(0);
const availableTags = ["Vegetarian", "Vegan", "Gluten-Free", "Low-Carb", "Keto", "Paleo"];
const selectedTags = ref([]);
const uploadedImages = ref([]);

function toggleTag(tag) {
    const index = selectedTags.value.indexOf(tag);
    if (index === -1) {
        selectedTags.value.push(tag);
    } else {
        selectedTags.value.splice(index, 1);
    }
}

function handleImageUpload(event) {
    const files = event.target.files;
    uploadedImages.value = [];

    for (let i = 0; i < files.length; i++) {
        console.log(files[i]);
        const reader = new FileReader();
        reader.onload = function (e) {
            uploadedImages.value.push(e.target.result.split(",")[1]); // Store base64 encoded string without mime type
        };
        reader.readAsDataURL(files[i]);
    }
}

function submitRecipe() {
    // Here you would typically send the data to your backend
    const recipeData = {
        title: title.value,
        description: description.value,
        ingredients: ingredients.value,
        tags: selectedTags.value,
        prepTime: prepTime.value,
        images: uploadedImages.value,
    };

    console.log("Submitting recipe:", recipeData);

    // Reset form after submission
    title.value = "";
    description.value = "";
    ingredients.value = "";
    selectedTags.value = [];
    prepTime.value = 0;
    uploadedImages.value = [];

    // Navigate to a different route (e.g., recipe list) after submission
    router.push("/recipes");
}
function goToHome() {
    router.push({ name: "home", query: { canisterId: route.query.canisterId } });
}
</script>

<template>
    <div class="flex min-h-screen w-full items-center justify-center bg-gray-800 p-4 text-gray-100">
        <div class="w-full max-w-3xl rounded-lg bg-gray-700 p-8 shadow-xl">
            <h1 class="mb-8 text-center text-3xl font-bold text-indigo-300">Add New Recipe</h1>
            <form @submit.prevent="submitRecipe" class="space-y-6">
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
                    <label for="description" class="mb-2 block text-lg font-medium text-gray-300">Description</label>
                    <textarea
                        v-model="description"
                        id="description"
                        rows="4"
                        required
                        class="w-full rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                    ></textarea>
                </div>

                <div>
                    <label for="ingredients" class="mb-2 block text-lg font-medium text-gray-300">Ingredients</label>
                    <textarea
                        v-model="ingredients"
                        id="ingredients"
                        rows="6"
                        class="w-full rounded-md border-gray-500 bg-gray-600 px-4 py-3 text-lg text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"
                    ></textarea>
                </div>

                <div>
                    <label class="mb-2 block text-lg font-medium text-gray-300">Tags</label>
                    <div class="flex flex-wrap justify-center gap-3">
                        <button
                            v-for="tag in availableTags"
                            :key="tag"
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
                    <label for="images" class="mb-2 block text-lg font-medium text-gray-300">Upload Images</label>
                    <input
                        type="file"
                        id="images"
                        @change="handleImageUpload"
                        multiple
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
                    <span class="text-lg text-gray-400">{{ uploadedImages.length }} image(s) selected</span>
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
</template>
