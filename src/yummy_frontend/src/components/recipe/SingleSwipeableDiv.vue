<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
const emit = defineEmits(["itemClick"]);

const props = defineProps({
    recipes: {
        type: Array,
        required: true,
    },
});

const carouselRef = ref(null);
const currentOffset = ref(0);
let containerWidth = ref(0);

let startX = 0;
let currentX = 0;
let temporaryX = 0;
let startOffset = 0;

const itemWidth = 300;

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
    if (Math.abs(temporaryDiff) > 3) {
        const diff = startX - currentX;
        const newOffset = startOffset + diff;
        currentOffset.value = Math.max(0, Math.min(newOffset, maxOffset.value));
        temporaryX = currentX;
    }
}

function onTouchEnd() {
    const diff = startX - currentX;
    currentOffset.value = startOffset;
    if (Math.abs(diff) > 50) {
        if (diff > 0) {
            moveCarousel(1);
        } else {
            moveCarousel(-1);
        }
    }
}

function moveCarousel(direction) {
    const visibleItems = Math.floor(containerWidth.value / itemWidth);
    const moveAmount = direction * ((visibleItems * itemWidth) / 2);
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
    <div class="relative mx-auto mb-[80px] mt-[30px] overflow-hidden">
        <div
            ref="carouselRef"
            class="flex transition-transform duration-300 ease-in-out"
            :style="{ transform: `translateX(${-currentOffset}px)` }"
            @touchstart="onTouchStart"
            @touchmove="onTouchMove"
            @touchend="onTouchEnd"
        >
            <div
                v-for="(item, index) in recipes"
                v-if="recipes"
                :key="index"
                class="w-[300px] flex-shrink-0 cursor-pointer p-4"
                @click="onItemClick(item)"
            >
                <div
                    class="flex h-[300px] flex-col items-center justify-start overflow-hidden rounded-[20px] bg-[#7f7f7f]"
                >
                    <div class="h-3/5 w-full bg-cover" :style="{ backgroundImage: `url(${item.mainImage})` }"></div>
                    <p class="p-2 text-lg text-white">
                        {{ item.name }}
                    </p>
                    <div class="mt-auto flex items-center self-end p-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            class="h-5 w-5"
                            fill="none"
                            stroke="#ddd"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <circle cx="12" cy="12" r="10"></circle>
                            <polyline points="12 6 12 12 16 14"></polyline>
                        </svg>

                        <p class="px-2 text-[14px] text-white">{{ item.totalTime }} mins</p>
                    </div>
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
