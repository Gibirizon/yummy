<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../../store/auth";
import Message from "../Message.vue";

const emit = defineEmits(["go-to-home"]);
const authStore = useAuthStore();
const { whoamiActor } = storeToRefs(authStore);

const router = useRouter();
const route = useRoute();
const newUsername = ref(""); // Initialize with current username
const id_from_route = BigInt(route.params.id);

const showMessage = ref(false);
const messageText = ref("");
const messageType = ref("");
async function saveChanges() {
    messageText.value = "Saving new username...";
    messageType.value = "info";
    showMessage.value = true;
    await whoamiActor.value.update_username(id_from_route, newUsername.value);
    goToInfo();
}

function goToInfo() {
    router.push({
        name: "profile-info",
        query: { canisterId: route.query.canisterId },
        params: { id: id_from_route },
    });
}
function cancelChanges() {
    emit("go-to-home");
}

function closeMessage() {
    showMessage.value = false;
}
</script>
<template>
    <div class="relative contain-content">
        <Transition name="slide">
            <Message v-if="showMessage" :text="messageText" :type="messageType" @close="closeMessage" />
        </Transition>
        <div
            class="profile-edit flex min-h-screen w-full flex-col items-center justify-center bg-gray-900 p-8 text-white"
        >
            <div class="w-full max-w-2xl">
                <h1 class="mb-8 text-center text-4xl font-bold">Edit Profile</h1>
                <div class="mb-8 rounded-lg bg-gray-800 px-8 py-10 shadow-lg">
                    <div class="mb-8">
                        <form @submit.prevent="saveChanges">
                            <label class="mb-4 block text-xl font-semibold" for="username"> Username </label>
                            <input
                                v-model="newUsername"
                                class="w-full rounded-lg bg-gray-700 px-6 py-4 text-xl leading-tight text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                id="username"
                                type="text"
                                placeholder="Enter new username"
                            />
                        </form>
                    </div>
                    <div class="flex flex-col space-y-4">
                        <button
                            @click="saveChanges"
                            class="w-full transform rounded-lg bg-green-700 px-6 py-4 text-xl font-bold text-white transition duration-300 ease-in-out hover:scale-105 hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50"
                        >
                            Save Changes
                        </button>
                        <button
                            @click="cancelChanges"
                            class="w-full transform rounded-lg bg-red-700 px-6 py-4 text-xl font-bold text-white transition duration-300 ease-in-out hover:scale-105 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
