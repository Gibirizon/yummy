<script setup>
const emit = defineEmits(["show-menu"]);
function showMenu() {
    emit("show-menu");
}
</script>

<template>
    <nav class="fixed left-0 top-0 z-[110] w-full bg-gray-300">
        <div class="relative z-[100] flex h-20 w-full items-center justify-between p-4">
            <div class="flex items-center gap-4">
                <a href="#" class="flex items-center" @click="showMenu">
                    <font-awesome-icon :icon="['fas', 'bars']" class="text-2xl" />
                </a>
                <h2 class="text-2xl">Yummy</h2>
            </div>
            <div>
                <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-2xl" />
            </div>
        </div>
    </nav>
</template>
