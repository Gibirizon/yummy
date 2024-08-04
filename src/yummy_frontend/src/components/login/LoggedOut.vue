<script setup>
import Username from "./Username.vue";
import { X } from "lucide-vue-next";
import { useAuthStore } from "./../../store/auth";
import { ref } from "vue";
import Message from "../Message.vue";
import { storeToRefs } from "pinia";
const emit = defineEmits(["close", "logged-in"]);

const signMethod = ref("Sign in");

const authStore = useAuthStore();
const { whoamiActor } = storeToRefs(authStore);
const showMessage = ref(false);
const messageText = ref("");
const messageType = ref("");
const signProcess = ref(true);

function changeSignMethod() {
    if (signMethod.value === "Sign in") {
        signMethod.value = "Sign up";
    } else {
        signMethod.value = "Sign in";
    }
}

async function SignIn() {
    try {
        await authStore.login();

        // Login successful, perform next actions
        if (!whoamiActor.value) {
            createMessage("Login failed - please try again", "warning");
            return;
        }
        let user_index = await whoamiActor.value.get_user_index();
        if (user_index.Err) {
            createMessage("You have signed up - set your username", "info");
            signProcess.value = false;
            return;
        }
        emit("logged-in", user_index.Ok);
    } catch (err) {
        // Handle login error
        console.error("Login error:", err);
        createMessage("Login failed - please try again", "error");
    }
}

async function creatingNewUser(username) {
    if (!whoamiActor.value) {
        signProcess.value = true;
        createMessage("Login failed - please try again", "warning");
        return;
    }
    createMessage("Creating new user...", "info");
    let new_user_index = await whoamiActor.value.create_user(username);
    signProcess.value = true;
    if (new_user_index.Err) {
        createMessage("You already have an account - logging in...", "info");
        let user_index = await whoamiActor.value.get_user_index();
        emit("logged-in", user_index.Ok);
        return;
    }
    emit("logged-in", new_user_index.Ok);
}
function createMessage(text, type) {
    messageText.value = text;
    messageType.value = type;
    showMessage.value = true;
}
const closeMessage = () => {
    showMessage.value = false;
};
</script>
<template>
    <div class="fixed left-0 top-0 z-[200] h-full w-full bg-transparent backdrop-blur-sm"></div>
    <Transition name="slide">
        <Message v-if="showMessage" :text="messageText" :type="messageType" @close="closeMessage" />
    </Transition>
    <div v-if="signProcess" class="sing-in-box fixed bottom-0 z-[210] w-full rounded-t-3xl bg-[#1b1c21]">
        <button
            @click="emit('close')"
            class="absolute right-4 top-2 rounded-full p-2 transition-colors duration-200 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-600"
        >
            <X class="h-5 w-5 text-gray-300" />
        </button>
        <div class="flex flex-col items-center justify-around gap-2 p-6 text-white">
            <h4 class="text-2xl font-semibold">{{ signMethod }} to Yummy</h4>
            <p class="font-thin">Where you can look for thousands of recipes</p>
            <div class="m-4 flex items-center">
                <div class="flex min-h-[45px] w-[60px] min-w-[60px] items-center justify-center bg-[#242834] p-2">
                    <img src="/images/ic_logo.svg" alt="Internet Computer logo" />
                </div>
                <button type="button" class="min-h-[45px] bg-[#085d8c] p-2 hover:bg-[#053854]" @click="SignIn">
                    <span> {{ signMethod }} with Internet Identity </span>
                </button>
            </div>
            <div v-if="signMethod === 'Sign in'">
                <p>
                    Don't have an account?
                    <span class="text-[#2199e2] hover:underline"
                        ><a href="#" @click="changeSignMethod">Sign up</a></span
                    >
                </p>
            </div>
            <div v-else>
                <p>
                    Already have an account?
                    <span class="text-[#2199e2] hover:underline"
                        ><a href="#" @click="changeSignMethod">Sign in</a></span
                    >
                </p>
            </div>
        </div>
    </div>
    <Username v-else @new-user-created="creatingNewUser" />
</template>

<style scoped>
@media only screen and (min-width: 768px) {
    .sing-in-box {
        @apply bottom-1/2 left-1/2 w-1/2 max-w-[500px] -translate-x-1/2 translate-y-1/2 rounded-b-3xl;
    }
}
</style>
