<script setup>
import LoggedOut from "./login/LoggedOut.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { ref, onMounted, onUnmounted } from "vue";

const whoami = ref("");
function whoamiCall() {
    if (authStore.whoamiActor) {
        authStore.whoamiActor?.whoami().then((res) => (whoami.value = res));
    } else {
        whoami.value = "You are not logged in";
    }
}

const authStore = useAuthStore();
const { isReady, isAuthenticated } = storeToRefs(authStore);
if (isReady.value === false) {
    authStore.init();
}
const loggingProcess = ref(false);

const topNavBarIsPrimary = ref(true);
const widthGreaterThanMobile = ref(false);
let mql;

function toggleSideNavBar() {
    topNavBarIsPrimary.value = false;
}

function hideSideNavBar() {
    topNavBarIsPrimary.value = true;
}

function handleWidthChange(e) {
    widthGreaterThanMobile.value = e.matches;
    if (widthGreaterThanMobile.value) {
        topNavBarIsPrimary.value = false;
    } else {
        topNavBarIsPrimary.value = true;
    }
}

onMounted(() => {
    mql = window.matchMedia("(min-width: 992px)");
    handleWidthChange(mql);
    mql.addEventListener("change", handleWidthChange);
});

onUnmounted(() => {
    mql.removeEventListener("change", handleWidthChange);
});
</script>

<template>
    <div
        :class="[
            topNavBarIsPrimary || widthGreaterThanMobile
                ? 'h-0 w-0'
                : 'fixed z-10 h-full w-full bg-black/40 opacity-100',
        ]"
        @click="hideSideNavBar"
    ></div>
    <div
        :class="[topNavBarIsPrimary ? '-translate-x-full' : 'translate-x-0']"
        class="duration-400 fixed left-0 top-0 z-20 flex h-screen w-60 flex-col items-center gap-4 bg-white p-4 shadow-lg transition-transform"
    >
        <h2 class="p-2 text-2xl">Yummy</h2>
        <div v-if="isReady">
            <button
                v-if="isAuthenticated"
                @click="async () => await authStore.logout()"
                type="button"
                class="login-button"
            >
                Sign out
            </button>
            <button v-else @click="loggingProcess = true" type="button" class="login-button">Sign in</button>
        </div>
        <button @click="whoamiCall" type="button" class="login-button">Who am I?</button>
        <p>{{ whoami }}</p>
    </div>
    <nav
        class="fixed h-20 w-screen items-center justify-between bg-gray-300 p-8"
        :class="widthGreaterThanMobile ? 'hidden' : 'flex'"
    >
        <div class="flex items-center gap-4">
            <a href="#" class="flex items-center" @click="toggleSideNavBar">
                <font-awesome-icon :icon="['fas', 'bars']" class="text-2xl" />
            </a>
            <h2 class="text-2xl">Yummy</h2>
        </div>
        <div>
            <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-2xl" />
        </div>
    </nav>
    <LoggedOut @finish-logging-in="loggingProcess = false" v-if="loggingProcess && !isAuthenticated" />
</template>
