<script setup>
import UsernameBox from "./login/Username.vue";
import LoggedOut from "./login/LoggedOut.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { ref, onMounted, onUnmounted } from "vue";
import { watch } from "vue";
import { RouterLink } from "vue-router";
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

const whoami = ref("");
const leftMenuIsVisible = ref(false);
const usernameBoxIsVisible = ref(false);

const authStore = useAuthStore();
const { isReady, isAuthenticated } = storeToRefs(authStore);
if (isReady.value === false) {
    authStore.init();
}
const loggingProcess = ref(false);

watch(
    () => props.isVisible,
    (_) => {
        leftMenuIsVisible.value = !leftMenuIsVisible.value;
    }
);

function whoamiCall() {
    console.log(authStore.whoamiActor);
    if (authStore.whoamiActor) {
        authStore.whoamiActor?.whoami().then((res) => (whoami.value = res));
    } else {
        whoami.value = "You are not logged in";
    }
}

function sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time));
}
async function finishSignUp() {
    while (!authStore.isAuthenticated) {
        await sleep(100);
    }
    console.log(authStore.whoamiActor);

    loggingProcess.value = false;
    let id;
    await authStore.whoamiActor?.whoami().then((res) => (id = res));
    console.log(id);
    await authStore.whoamiActor?.get_user(id).then((e) => {
        if (Object.keys(e)[0] === "Err") {
            console.log(e.Err);
            usernameBoxIsVisible.value = true;
        }
    });
}

onMounted(() => {
    window.addEventListener("message", (event) => {
        if (event.data.kind === "authorize-client-success") {
            finishSignUp();
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
    <UsernameBox @new-user-created="usernameBoxIsVisible = false" v-if="usernameBoxIsVisible" />
    <div
        class="duration-400 left-menu fixed left-0 top-0 z-[130] flex h-screen w-[240px] flex-col items-center gap-4 bg-[#3e3e3e] p-4 shadow-lg transition-transform"
        :class="leftMenuIsVisible ? 'translate-x-0' : '-translate-x-full'"
    >
        <h2 class="p-2 text-2xl text-white">Yummy</h2>
        <div v-if="isReady">
            <button
                v-if="isAuthenticated"
                @click="async () => await authStore.logout()"
                type="button"
                class="login-button"
            >
                <span class="py-[10px]">Sign out</span>
            </button>
            <button v-else @click="loggingProcess = true" type="button" class="login-button">
                <span>Sign in</span>
            </button>
        </div>
        <button @click="whoamiCall" type="button" class="login-button"><span>Who am I</span></button>
        <p>{{ whoami }}</p>
        <div>
            <ul class="space-y-6 text-[18px]">
                <li>
                    <RouterLink to="/" class="flex items-center gap-3 text-gray-300 hover:text-white">
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
                <li>
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
                        <li><a href="#" class="text-gray-400 hover:text-white">View Profile</a></li>
                        <li><a href="#" class="text-gray-400 hover:text-white">Edit Profile</a></li>
                    </ul>
                </li>
                <li>
                    <RouterLink to="/about" class="flex items-center gap-3 text-gray-300 hover:text-white">
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
    </div>
    <LoggedOut @finish-logging-in="loggingProcess = false" v-if="loggingProcess && !isAuthenticated" />
</template>

<style scoped>
@media screen and (min-width: 992px) {
    .left-menu {
        @apply sticky translate-x-0;
    }
}
</style>
