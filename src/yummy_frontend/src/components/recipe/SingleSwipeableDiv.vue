<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import RecipeCard from "./RecipeCard.vue";
const emit = defineEmits(["itemClick"]);

const props = defineProps({
    recipes: {
        type: Array,
        required: true,
    },
});

const carouselRef = ref(null);
const currentOffset = ref(0);
const containerWidth = ref(0);

let startX = 0;
let currentX = 0;
let temporaryX = 0;
let startOffset = 0;
let itemWidth = 300;
let multiplier = 1.3;

const totalWidth = computed(() => props.recipes.length * itemWidth);
const maxOffset = computed(() => Math.max(0, totalWidth.value - containerWidth.value));

const isLastItemVisible = computed(() => {
    return currentOffset.value + containerWidth.value >= totalWidth.value;
});

function onTouchStart(e) {
    startX = e.touches[0].clientX;
    temporaryX = startX;
    startOffset = currentOffset.value;
}

function onTouchMove(e) {
    currentX = e.touches[0].clientX;
    const temporaryDiff = temporaryX - currentX;
    if (Math.abs(temporaryDiff) > 1) {
        currentOffset.value = Math.max(0, Math.min(currentOffset.value + temporaryDiff, maxOffset.value));
        temporaryX = currentX;
    }
}

function onTouchEnd() {
    const diff = startX - currentX;
    currentOffset.value = startOffset;
    if (Math.abs(diff) > window.innerWidth / 5) {
        moveCarousel(diff / (window.innerWidth / 5));
    }
}

function moveCarousel(direction) {
    const visibleItems = Math.floor(containerWidth.value / itemWidth);
    const moveAmount = direction * ((visibleItems * itemWidth) / multiplier);
    let newOffset = currentOffset.value + moveAmount;

    // Ensure the last item is fully visible when moving forward
    if (direction > 0 && newOffset + containerWidth.value > totalWidth.value) {
        newOffset = maxOffset.value;
    }

    // Ensure we don't scroll past the beginning when moving backward
    newOffset = Math.max(0, newOffset);

    currentOffset.value = newOffset;
}

const onItemClick = (item) => {
    emit("itemClick", item);
};

function getContainerWidth() {
    if (window.innerWidth < 1024) {
        itemWidth = 300;
        multiplier = 1.3;
    } else {
        itemWidth = 400;
        multiplier = 1.5;
    }
    containerWidth.value = carouselRef.value.offsetWidth;
}

onMounted(() => {
    window.addEventListener("resize", getContainerWidth);
    getContainerWidth();
});

onUnmounted(() => {
    window.removeEventListener("resize", getContainerWidth);
});
</script>

<template>
    <div class="relative mx-auto mb-[40px] mt-[15px] overflow-hidden px-2 sm:mb-[80px] sm:mt-[30px] sm:px-4">
        <div
            ref="carouselRef"
            class="flex w-full transition-transform duration-200 ease-in-out"
            :style="{ transform: `translateX(${-currentOffset}px)` }"
            @touchstart="onTouchStart"
            @touchmove="onTouchMove"
            @touchend="onTouchEnd"
        >
            <div
                v-for="(recipe, index) in recipes"
                v-if="recipes"
                :key="index"
                class="w-[300px] flex-shrink-0 cursor-pointer p-2 lg:w-[400px]"
                @click="onItemClick(recipe)"
            >
                <div class="flex h-full flex-col">
                    <RecipeCard :recipe="recipe" class="flex-grow" />
                </div>
            </div>
        </div>

        <button
            @click="moveCarousel(-1)"
            class="duration-400 absolute left-4 top-1/2 -translate-y-1/2 transform rounded-full bg-white bg-opacity-50 p-2 shadow-md transition-all hover:bg-opacity-75"
            :class="{ invisible: currentOffset === 0 }"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
        </button>
        <button
            @click="moveCarousel(1)"
            class="duration-400 absolute right-4 top-1/2 -translate-y-1/2 transform rounded-full bg-white bg-opacity-50 p-2 shadow-md transition-all hover:bg-opacity-75"
            :class="{ invisible: isLastItemVisible }"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
        </button>
    </div>
</template>
