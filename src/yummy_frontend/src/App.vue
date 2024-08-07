<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { useMessageStore } from "./store/message";
import Message from "./components/Message.vue";
import Nav from "./components/Nav.vue";
import LeftMenu from "./components/LeftMenu.vue";
import Search from "./components/Search.vue";

const messageStore = useMessageStore();
const isLeftMenuVisible = ref(false);
const isSearchVisible = ref(false);

function toggleSearch() {
    isSearchVisible.value = !isSearchVisible.value;
}
// Add event listener for Ctrl+K
onMounted(() => {
    window.addEventListener("keydown", (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === "k") {
            e.preventDefault();
            toggleSearch();
        }
    });
});

onUnmounted(() => {
    window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
    <div class="relative">
        <Search :isVisible="isSearchVisible" @close="isSearchVisible = false" />
        <Transition name="slide" class="w-full">
            <Message
                v-if="messageStore.isVisible"
                :text="messageStore.text"
                :type="messageStore.type"
                :duration="messageStore.duration"
                @close="messageStore.hideMessage"
            />
        </Transition>
        <div class="main-container flex w-full flex-row">
            <LeftMenu :isVisible="isLeftMenuVisible" @toggle-search="toggleSearch" @close="isLeftMenuVisible = false" />
            <div
                class="relative flex w-full flex-1 flex-col items-center overflow-y-auto overflow-x-hidden pt-[80px] contain-content lg:pt-0"
            >
                <Nav class="lg:hidden" @toggle-search="toggleSearch" @show-menu="isLeftMenuVisible = true" />
                <main class="relative w-full bg-gray-800">
                    <RouterView />
                </main>
            </div>
        </div>
    </div>
</template>
