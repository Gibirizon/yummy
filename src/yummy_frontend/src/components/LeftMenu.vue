<script setup>
import UsernameBox from "./login/Username.vue";
import LoggedOut from "./login/LoggedOut.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../store/auth";
import { ref, onMounted, onUnmounted } from "vue";
import { watch } from "vue";

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
        class="duration-400 left-menu fixed left-0 top-0 z-[130] flex h-screen w-60 flex-col items-center gap-4 bg-[#343434] p-4 shadow-lg transition-transform"
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
                Sign out
            </button>
            <button v-else @click="loggingProcess = true" type="button" class="login-button">Sign in</button>
        </div>
        <button @click="whoamiCall" type="button" class="login-button">Who am I?</button>
        <p>{{ whoami }}</p>
    </div>
    <LoggedOut @finish-logging-in="loggingProcess = false" v-if="loggingProcess && !isAuthenticated" />
</template>

<style scoped>
@media screen and (min-width: 992px) {
    .left-menu {
        @apply relative translate-x-0;
    }
}
</style>
