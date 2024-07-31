<script setup>
import { onMounted, onUnmounted } from "vue";

const props = defineProps({
    text: {
        type: String,
        required: true,
    },
    type: {
        type: String,
        default: "info",
        validator: (value) => ["success", "error", "info", "warning"].includes(value),
    },
    duration: {
        type: Number,
        default: 5000,
    },
});

const emit = defineEmits(["close"]);

let timer = null;

const typeClasses = {
    success: "bg-green-900 text-green-100",
    error: "bg-red-900 text-red-100",
    info: "bg-blue-900 text-blue-100",
    warning: "bg-yellow-900 text-yellow-100",
};

const textClasses = {
    success: "text-green-100",
    error: "text-red-100",
    info: "text-blue-100",
    warning: "text-yellow-100",
};

const close = () => {
    emit("close");
};

onMounted(() => {
    if (props.duration > 0) {
        timer = setTimeout(close, props.duration);
    }
});

onUnmounted(() => {
    if (timer) {
        clearTimeout(timer);
    }
});
</script>

<template>
    <div class="fixed left-0 right-0 top-0 z-[500] flex items-start justify-center p-4">
        <div
            :class="[
                'pointer-events-auto flex w-full max-w-sm rounded-lg shadow-lg ring-1 ring-black ring-opacity-5',
                typeClasses[props.type],
            ]"
        >
            <div class="w-0 flex-1 p-4">
                <div class="flex items-start">
                    <div class="ml-3 flex-1">
                        <p :class="['text-lg font-medium', textClasses[props.type]]">
                            {{ props.text }}
                        </p>
                    </div>
                </div>
            </div>
            <div class="flex border-gray-700">
                <button
                    @click="close"
                    class="flex w-full items-center justify-center rounded-none rounded-r-lg border border-transparent p-4 text-sm font-medium hover:scale-125 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:scale-100 transition-transform"
                >
                    <svg
                        class="h-5 w-5"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        aria-hidden="true"
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
        </div>
    </div>
</template>
