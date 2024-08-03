<script setup>
import { ref, computed } from "vue";
import { Clock, ChefHat, Tag, ChevronDown, ChevronUp, Utensils, List } from "lucide-vue-next";

const recipe = ref({
    title: "Spicy Thai Basil Chicken",
    image: "",
    instructions: [
        "Heat oil in a large skillet over medium-high heat.",
        "Add garlic and chili, stir-fry for 30 seconds.",
        "Add chicken and stir-fry until it loses its raw color.",
        "Add the sauce mixture and continue to stir-fry until the chicken is cooked.",
        "Remove from heat and stir in the basil leaves.",
        "Serve hot with steamed rice.",
    ],
    ingredients: [
        "1 lb boneless, skinless chicken thighs, sliced",
        "4 cloves garlic, minced",
        "2-3 Thai chili peppers, finely chopped",
        "1/4 cup soy sauce",
        "2 tsp oyster sauce",
        "1 tsp sugar",
        "1 cup Thai basil leaves",
        "2 tbsp vegetable oil",
    ],
    tags: ["Spicy", "Quick", "Stir-fry"],
    prepTime: 30,
    cuisines: ["Thai", "Asian"],
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
</script>

<template>
    <div class="min-h-screen w-full bg-gray-800 p-6 text-gray-100 md:p-10">
        <div class="mx-auto max-w-6xl overflow-hidden rounded-xl bg-gray-700 shadow-xl">
            <img :src="recipe.image" :alt="recipe.title" class="h-96 w-full object-cover" />
            <div class="p-8 md:p-10">
                <h1 class="mb-6 text-4xl font-bold text-indigo-300 md:text-5xl">
                    {{ recipe.title }}
                </h1>

                <div class="mb-8 flex flex-wrap items-center gap-6">
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
