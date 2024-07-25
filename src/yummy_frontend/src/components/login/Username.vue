<script setup>
import { ref } from "vue";
import { useAuthStore } from "./../../store/auth";

const authStore = useAuthStore();
const username = ref("");
const emit = defineEmits(["new-user-created"]);

async function createNewUser() {
    emit("new-user-created");
    if (authStore.whoamiActor) {
        await authStore.whoamiActor?.create_user(username.value).then((e) => {
            console.log(e);
        });
    }
}
</script>

<template>
    <div class="absolute z-[200] h-full w-full bg-transparent backdrop-blur-sm"></div>
    <div class="enter-your-name-box fixed bottom-0 z-[210] w-full rounded-t-3xl bg-[#1b1c21]">
        <div class="flex flex-col items-center justify-around gap-4 p-4 text-white">
            <h4 class="text-2xl font-semibold">Finish signing up</h4>
            <form
                @submit.prevent="createNewUser"
                action=""
                class="flex w-full flex-col items-center justify-center gap-4"
            >
                <div class="flex w-full items-center justify-around">
                    <label for="username" class="text-md">Enter your name:</label>
                    <input
                        type="text"
                        name="username"
                        placeholder="username"
                        v-model="username"
                        class="w-[200px] rounded border-2 border-[#2083df] p-3 text-sm text-gray-700 placeholder-gray-400 shadow outline-none focus:border-[#0000CD] focus:outline-none"
                    />
                </div>
                <button class="rounded-xl bg-blue-700 px-10 py-3 text-white hover:bg-blue-600" type="submit">
                    Continue
                </button>
            </form>
        </div>
    </div>
</template>

<style scoped>
@media only screen and (min-width: 576px) {
    .enter-your-name-box {
        @apply bottom-1/2 left-1/2 w-3/4 max-w-[500px] -translate-x-1/2 translate-y-1/2 rounded-b-3xl;
    }
}
</style>
