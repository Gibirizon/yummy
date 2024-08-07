<script setup>
import { ref } from "vue";
import { UserCircle, ArrowRight } from "lucide-vue-next";

const username = ref("");
const emit = defineEmits(["new-user-created"]);

async function createNewUser() {
    emit("new-user-created", username.value);
}
</script>

<template>
    <div class="enter-your-name-box fixed bottom-0 z-[210] w-full rounded-t-3xl bg-gray-800 shadow-lg">
        <div class="flex flex-col items-center justify-around gap-6 p-8 text-white">
            <h4 class="text-3xl font-bold">Finish signing up</h4>
            <form @submit.prevent="createNewUser" class="flex w-full flex-col items-center justify-center gap-6">
                <div class="relative w-full">
                    <label for="username" class="mb-2 block text-sm font-medium text-gray-300">Enter your name:</label>
                    <div class="relative">
                        <UserCircle
                            class="absolute left-3 top-1/2 -translate-y-1/2 transform text-gray-400"
                            size="20"
                        />
                        <input
                            type="text"
                            required
                            minlength="1"
                            name="username"
                            placeholder="Username"
                            v-model="username"
                            maxlength="30"
                            class="w-full rounded-lg border-2 border-gray-600 bg-gray-700 p-3 pl-10 text-sm text-white placeholder-gray-400 shadow-sm outline-none transition-colors focus:border-blue-500 focus:ring focus:ring-blue-200 focus:ring-opacity-50"
                        />
                    </div>
                </div>
                <button
                    class="group flex items-center justify-center rounded-xl bg-blue-600 px-6 py-3 text-white transition-all hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:ring-offset-gray-800"
                    type="submit"
                >
                    Continue
                    <ArrowRight class="ml-2 transition-transform group-hover:translate-x-1" size="20" />
                </button>
            </form>
        </div>
    </div>
</template>

<style scoped>
.enter-your-name-box {
    @apply transition-all duration-300 ease-in-out;
}

@media only screen and (min-width: 576px) {
    .enter-your-name-box {
        @apply bottom-1/2 left-1/2 w-3/4 max-w-[500px] -translate-x-1/2 translate-y-1/2 rounded-3xl;
    }
}
</style>
