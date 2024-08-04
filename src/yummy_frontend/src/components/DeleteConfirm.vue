<script setup>
const emit = defineEmits(["close", "confirm"]);
</script>
<template>
    <div class="fixed inset-0 z-50 flex items-center justify-center">
        <div @click="emit('close')" class="absolute inset-0 bg-black bg-opacity-75 backdrop-blur-sm"></div>
        <div class="relative z-10 w-full max-w-md overflow-hidden rounded-lg bg-gray-800 shadow-xl">
            <div class="p-6">
                <h3 class="mb-4 text-2xl font-bold text-white">Confirm Delete</h3>
                <p class="mb-6 text-gray-300">
                    Are you sure you want to delete your profile? This action cannot be undone.
                </p>
                <div class="flex justify-end space-x-4">
                    <button
                        @click="emit('close')"
                        class="rounded-lg bg-gray-600 px-4 py-2 font-bold text-white transition-colors hover:bg-gray-500"
                    >
                        No, Cancel
                    </button>
                    <button
                        @click="emit('confirm')"
                        class="rounded-lg bg-red-600 px-4 py-2 font-bold text-white transition-colors hover:bg-red-700"
                    >
                        Yes, Delete
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
