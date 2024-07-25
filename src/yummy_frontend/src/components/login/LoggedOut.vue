<script setup>
import { useAuthStore } from "./../../store/auth";
import { ref } from "vue";
const emit = defineEmits(["finish-logging-in"]);

const signMethod = ref("Sign in");

const authStore = useAuthStore();

function changeSignMethod() {
    console.log(signMethod);
    if (signMethod.value === "Sign in") {
        signMethod.value = "Sign up";
    } else {
        signMethod.value = "Sign in";
    }
}
async function SignIn() {
    await authStore.login();
    console.log("in sign in");
    emit("finish-logging-in");
}
</script>
<template>
    <div class="absolute z-[200] h-full w-full bg-transparent backdrop-blur-sm"></div>
    <div class="sing-in-box fixed bottom-0 z-[210] w-full rounded-t-3xl bg-[#1b1c21]">
        <button
            type="button"
            class="absolute right-4 top-2 h-[30px] w-[30px] rounded-full text-white hover:bg-[#2f313a]"
            @click="emit('finish-logging-in')"
        >
            <font-awesome-icon :icon="['fas', 'xmark']" />
        </button>
        <div class="flex flex-col items-center justify-around gap-2 p-6 text-white">
            <h4 class="text-2xl font-semibold">{{ signMethod }} to Yummy</h4>
            <p class="font-thin">Where you can look for thousands of recipes</p>
            <div class="m-4 flex items-center">
                <div class="flex min-h-[45px] w-[60px] min-w-[60px] items-center justify-center bg-[#242834] p-2">
                    <img src="./../../../public/images/ic_logo.svg" alt="Internet Computer logo" />
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
</template>

<style scoped>
@media only screen and (min-width: 768px) {
    .sing-in-box {
        @apply bottom-1/2 left-1/2 w-1/2 max-w-[500px] -translate-x-1/2 translate-y-1/2 rounded-b-3xl;
    }
}
</style>
