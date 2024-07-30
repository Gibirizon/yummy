<script setup>
import UsernameBox from "./login/Username.vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import LoggedOut from "./login/LoggedOut.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { ref, watch, onMounted, onUnmounted } from "vue";
import { RouterLink } from "vue-router";
import { faL } from "@fortawesome/free-solid-svg-icons";
import { computed } from "vue";
import { useRouter, useRoute } from "vue-router";

const route = useRoute();
const router = useRouter();

const canisterId = route.query.canisterId;

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

// const whoami = ref("");
const user_index = ref(0);
const leftMenuIsVisible = ref(false);
const usernameBoxIsVisible = ref(false);

const authStore = useAuthStore();
const { isReady, isAuthenticated, whoamiActor } = storeToRefs(authStore);
console.log(isAuthenticated, isReady);
init();
async function init() {
    console.log("isReady: ", isReady.value);
    if (isReady.value === false) {
        await authStore.init();
        console.log("isAuthenticated: ", isAuthenticated.value);
        await updateLoginStatus();
    }
}
const loggingProcess = ref(false);

watch(
    () => props.isVisible,
    (_) => {
        leftMenuIsVisible.value = !leftMenuIsVisible.value;
    }
);

function sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time));
}
async function finishSignUp() {
    console.log("!!!!!");
    console.log("finish sign up");
    console.log("finish sign up");
    console.log("isauthenticated", isAuthenticated.value);
    while (!isAuthenticated.value) {
        await sleep(100);
        console.log("isauthenticated", isAuthenticated.value);
    }
    console.log("whoami actor: ", whoamiActor.value);

    loggingProcess.value = false;
    // let id;
    // await whoamiActor.value.whoami().then((res) => (id = res));
    // whoami.value = id;
    await whoamiActor.value.get_user_index_by_principal().then((e) => {
        console.log("User index after signing up:", e);
        if (e.Err) {
            console.log(e.Err);
            usernameBoxIsVisible.value = true;
        } else {
            user_index.value = e.Ok;
            console.log("User index after signing up:", user_index.value);
        }
    });
}
async function signUserOut() {
    user_index.value = 0;
    await authStore.logout();
    // whoami.value = "";
}

async function changeUserId(id) {
    user_index.value = id;
    console.log(user_index.value);
}

async function updateLoginStatus() {
    // while (!isAuthenticated.value) {
    //     sleep(100);
    // }
    if (isAuthenticated.value) {
        console.log("updating login status");
        // await whoamiActor.value.whoami().then((res) => (whoami.value = res));
        await whoamiActor.value.get_user_index_by_principal().then((res) => {
            console.log("User index: ", res);
            user_index.value = res.Ok ? res.Ok : 0;
            console.log(user_index.value);
        });
    }
}
function goToNewRecipe() {
    router.push({ name: "new-recipe", query: { canisterId: canisterId } });
}
onMounted(() => {
    console.log(isAuthenticated);
    window.addEventListener("message", async (event) => {
        if (event.data.kind === "authorize-client-success") {
            console.log("finish sign up listener");
            await finishSignUp();
        }
    });
});

onUnmounted(() => {
    window.removeEventListener("message", (event) => {
        if (event.data.kind === "authorize-client-success") {
            finishSignUp();
        }
    });
});
</script>
<template>
    <div v-show="usernameBoxIsVisible">
        <UsernameBox @user-index="changeUserId" @new-user-created="usernameBoxIsVisible = false" />
    </div>
    <div
        class="duration-400 left-menu fixed left-0 top-0 z-[130] flex h-screen w-[240px] flex-col items-center gap-4 bg-[#3e3e3e] p-4 shadow-lg transition-transform"
        :class="leftMenuIsVisible ? 'translate-x-0' : '-translate-x-full'"
    >
        <h2 class="p-2 text-2xl text-white">Yummy</h2>
        <div v-if="isReady">
            <button v-if="isAuthenticated" @click="signUserOut" type="button" class="primary-button">
                <span class="py-[10px]">Sign out</span>
            </button>
            <button v-else @click="loggingProcess = true" type="button" class="primary-button">
                <span>Sign in</span>
            </button>
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
                    <ul v-if="isDropdownOpen('recipes')" class="ml-6 mt-2 space-y-2">
                        <li><a href="#" class="text-gray-400 hover:text-white">Browse</a></li>
                        <li><a href="#" class="text-gray-400 hover:text-white">Create New</a></li>
                        <li><a href="#" class="text-gray-400 hover:text-white">Favorites</a></li>
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
                    <ul v-if="isDropdownOpen('profile')" class="ml-6 mt-2 space-y-2">
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
        <!-- <div v-show="isAuthenticated"> -->
        <div class="flex justify-center">
            <button
                @click="goToNewRecipe"
                class="inline-flex items-center rounded-md border border-transparent bg-indigo-600 px-6 py-3 text-lg font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-800"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="mr-2 h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z"
                        clip-rule="evenodd"
                    />
                </svg>
                Add Recipe
            </button>
        </div>
        <!-- </div> -->
    </div>
    <div v-show="loggingProcess && !isAuthenticated">
        <LoggedOut @finish-logging-in="loggingProcess = false" />
    </div>
</template>

<style scoped>
@media screen and (min-width: 992px) {
    .left-menu {
        @apply sticky translate-x-0;
    }
}
</style>
