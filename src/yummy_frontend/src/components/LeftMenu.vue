<script setup>
import LoggedOut from "./login/LoggedOut.vue";
import { LogIn, LogOut } from "lucide-vue-next";
import Message from "./Message.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { ref, watch, computed } from "vue";
import { RouterLink } from "vue-router";
import { useRouter, useRoute } from "vue-router";

const route = useRoute();
const router = useRouter();

const canisterId = computed(() => route.query.canisterId);

const openDropdowns = ref([]);

function toggleDropdown(dropdown) {
    const index = openDropdowns.value.indexOf(dropdown);
    if (index === -1) {
        openDropdowns.value.push(dropdown);
    } else {
        openDropdowns.value.splice(index, 1);
    }
}

function isDropdownOpen(dropdown) {
    return openDropdowns.value.includes(dropdown);
}
const props = defineProps({
    isVisible: {
        type: Boolean,
        required: true,
    },
});

const user_index = ref(0);
const leftMenuIsVisible = ref(false);

const authStore = useAuthStore();
const { isReady, isAuthenticated } = storeToRefs(authStore);
init();
async function init() {
    console.log("isReady: ", isReady.value);
    if (isReady.value === false) {
        await authStore.init();
        console.log("isAuthenticated: ", isAuthenticated.value);
    }
    await updateLoginStatus();
}
const loggingProcess = ref(false);

watch(
    () => props.isVisible,
    (_) => {
        leftMenuIsVisible.value = !leftMenuIsVisible.value;
    }
);

async function signUserOut() {
    user_index.value = 0;
    await authStore.logout();
}

const showMessage = ref(false);
const messageText = ref("");
const messageType = ref("");
function createMessage(msg, type) {
    messageText.value = msg;
    messageType.value = type;
    showMessage.value = true;
}
function LoggedIn(index) {
    console.log("Logged in: ", index);
    loggingProcess.value = false;
    user_index.value = index;
    createMessage("Logged in successfuly", "success");
}

async function updateLoginStatus() {
    if (!isAuthenticated.value) {
        console.log("not authenticated");
        return;
    }
    console.log("updating login status");
    if (!authStore.whoamiActor) {
        console.log("no actor");
    }
    try {
        await authStore.whoamiActor?.get_user_index().then((res) => {
            console.log("User index: ", res);
            if (res.Err) {
                console.log("Error: ", res.Err);
                createMessage("You don't have account", "warning");
                signUserOut();
                return;
            }
            user_index.value = res.Ok ? res.Ok : 0;
            console.log(user_index.value);
        });
    } catch (error) {
        console.log("Error: ", error);
        await signUserOut();
        createMessage("Problems with identifying - login one more time", "error");
    }
}
function goToNewRecipe() {
    router.push({ name: "new-recipe", query: { canisterId: canisterId.value } });
}
function closeMessage() {
    showMessage.value = false;
}
</script>
<template>
    <Transition name="slide">
        <Message v-if="showMessage" :text="messageText" :type="messageType" @close="closeMessage" />
    </Transition>
    <div
        class="duration-400 left-menu fixed left-0 top-0 z-[130] flex h-screen w-[240px] flex-col items-center gap-4 bg-[#2C2F33] p-4 shadow-lg transition-transform"
        :class="leftMenuIsVisible ? 'translate-x-0' : '-translate-x-full'"
    >
        <h2 class="p-2 text-2xl text-white">Yummy</h2>
        <div v-if="isReady">
            <div class="flex justify-center">
                <button v-if="isAuthenticated" @click="signUserOut" type="button" class="left-menu-button">
                    <LogOut class="mr-2 h-6 w-6" />
                    Sign out
                </button>
                <button v-else @click="loggingProcess = true" type="button" class="left-menu-button">
                    <LogIn class="mr-2 h-6 w-6" />
                    Sign in
                </button>
            </div>
        </div>
        <div>
            <ul class="space-y-6 text-[18px]">
                <li>
                    <RouterLink
                        :to="{ name: 'home', query: { canisterId: canisterId } }"
                        class="flex items-center gap-3 text-gray-300 hover:text-white"
                    >
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
                                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                            />
                        </svg>
                        <span>Home</span>
                    </RouterLink>
                </li>
                <li>
                    <div
                        @click="toggleDropdown('recipes')"
                        class="flex cursor-pointer items-center justify-between text-gray-300 hover:text-white"
                    >
                        <div class="flex items-center gap-3">
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
                                    d="M19 21l-7-5-7 5V5a2 2 0 012-2h10a2 2 0 012 2v16z"
                                />
                            </svg>
                            <span>Recipes</span>
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4 transition-transform"
                                :class="{ 'rotate-180': isDropdownOpen('recipes') }"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M19 9l-7 7-7-7"
                                />
                            </svg>
                        </div>
                    </div>
                    <ul v-if="isDropdownOpen('recipes')" class="ml-6 mt-2 space-y-3">
                        <li v-if="isAuthenticated">
                            <RouterLink
                                :to="{
                                    name: 'recipes',
                                    query: { canisterId: canisterId },
                                    params: { type: 'Yours' },
                                }"
                                class="text-gray-400 hover:text-white"
                                >Your recipes</RouterLink
                            >
                        </li>
                        <li>
                            <RouterLink
                                :to="{
                                    name: 'recipes',
                                    query: { canisterId: canisterId },
                                    params: { type: 'Users' },
                                }"
                                class="text-gray-400 hover:text-white"
                                >Created by users</RouterLink
                            >
                        </li>
                        <li>
                            <RouterLink
                                :to="{
                                    name: 'recipes',
                                    query: { canisterId: canisterId },
                                    params: { type: 'Popular' },
                                }"
                                class="text-gray-400 hover:text-white"
                                >Popular</RouterLink
                            >
                        </li>
                        <li>
                            <div
                                @click="toggleDropdown('meals')"
                                class="flex cursor-pointer items-center justify-between text-gray-300 hover:text-white"
                            >
                                <div class="flex items-center gap-3">
                                    <span>Meals</span>
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="h-4 w-4 transition-transform"
                                        :class="{ 'rotate-180': isDropdownOpen('meals') }"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M19 9l-7 7-7-7"
                                        />
                                    </svg>
                                </div>
                            </div>
                            <ul v-if="isDropdownOpen('meals')" class="ml-6 mt-2 space-y-3">
                                <li>
                                    <RouterLink
                                        :to="{
                                            name: 'recipes',
                                            query: { canisterId: canisterId },
                                            params: { type: 'Breakfast' },
                                        }"
                                        class="text-gray-400 hover:text-white"
                                        >Breakfast</RouterLink
                                    >
                                </li>
                                <li>
                                    <RouterLink
                                        :to="{
                                            name: 'recipes',
                                            query: { canisterId: canisterId },
                                            params: { type: 'Dinner' },
                                        }"
                                        class="text-gray-400 hover:text-white"
                                        >Dinner</RouterLink
                                    >
                                </li>
                                <li>
                                    <RouterLink
                                        :to="{
                                            name: 'recipes',
                                            query: { canisterId: canisterId },
                                            params: { type: 'Dessert' },
                                        }"
                                        class="text-gray-400 hover:text-white"
                                        >Dessert</RouterLink
                                    >
                                </li>
                                <li>
                                    <RouterLink
                                        :to="{
                                            name: 'recipes',
                                            query: { canisterId: canisterId },
                                            params: { type: 'Snack' },
                                        }"
                                        class="text-gray-400 hover:text-white"
                                        >Snack</RouterLink
                                    >
                                </li>
                            </ul>
                        </li>
                    </ul>
                </li>
                <li v-show="isAuthenticated && user_index !== 0">
                    <div
                        @click="toggleDropdown('profile')"
                        class="flex cursor-pointer items-center justify-between text-gray-300 hover:text-white"
                    >
                        <div class="flex items-center gap-3">
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
                                    d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                />
                            </svg>
                            Profile
                        </div>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-4 w-4 transition-transform"
                            :class="{ 'rotate-180': isDropdownOpen('profile') }"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                        </svg>
                    </div>
                    <ul v-if="isDropdownOpen('profile')" class="ml-6 mt-2 space-y-3">
                        <li>
                            <RouterLink
                                :to="{
                                    name: 'profile-info',
                                    query: { canisterId: canisterId },
                                    params: { id: user_index },
                                }"
                                class="text-gray-400 hover:text-white"
                                >View Profile</RouterLink
                            >
                        </li>
                        <li>
                            <RouterLink
                                :to="{
                                    name: 'profile-edit',
                                    query: { canisterId: canisterId },
                                    params: { id: user_index },
                                }"
                                class="text-gray-400 hover:text-white"
                                >Edit Profile</RouterLink
                            >
                        </li>
                    </ul>
                </li>
                <li>
                    <RouterLink
                        :to="{ name: 'about', query: { canisterId: canisterId } }"
                        class="flex items-center gap-3 text-gray-300 hover:text-white"
                    >
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
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                        </svg>
                        About
                    </RouterLink>
                </li>
            </ul>
        </div>
        <div v-show="isAuthenticated" class="mt-4">
            <div class="flex justify-center">
                <button @click="goToNewRecipe" class="left-menu-button">
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
                    Create Recipe
                </button>
            </div>
        </div>
    </div>
    <div v-show="loggingProcess">
        <LoggedOut @logged-in="LoggedIn" @close="loggingProcess = false" />
    </div>
</template>

<style scoped>
@media screen and (min-width: 992px) {
    .left-menu {
        @apply sticky translate-x-0;
    }
}
</style>
