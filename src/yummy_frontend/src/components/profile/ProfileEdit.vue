<script setup>
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { storeToRefs } from "pinia";
import { useAuthStore } from "./../../store/auth";
import DeleteConfirm from "../DeleteConfirm.vue";
import { useMessageStore } from "../../store/message";
import { useDelete } from "../../composables/delete";

const emit = defineEmits(["go-to-home"]);
const messageStore = useMessageStore();
const authStore = useAuthStore();
const { whoamiActor } = storeToRefs(authStore);
const { showDeleteConfirmation, confirmDelete } = useDelete();

const router = useRouter();
const route = useRoute();
const newUsername = ref(""); // Initialize with current username
const id_from_route = BigInt(route.params.id);

async function saveChanges() {
    messageStore.showMessage("Saving changes...", "info");
    await whoamiActor.value.update_username(id_from_route, newUsername.value);
    messageStore.hideMessage();
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

async function confirmDel() {
    await confirmDelete(() => whoamiActor.value.delete_user());
    await authStore.logout();
    emit("go-to-home");
}
</script>

<template>
    <div class="relative">
        <div
            class="profile-edit flex min-h-screen w-full flex-col items-center justify-center bg-gray-900 p-8 text-white"
        >
            <div class="flex w-full max-w-2xl flex-col gap-4">
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
                <button
                    @click="showDeleteConfirmation = true"
                    class="flex items-center justify-center gap-2 self-end rounded-xl bg-red-600 p-4 text-white shadow-lg transition-all duration-300 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
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
                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                        />
                    </svg>
                    Delete account
                </button>
            </div>
        </div>
        <DeleteConfirm v-if="showDeleteConfirmation" @close="showDeleteConfirmation = false" @confirm="confirmDel" />
    </div>
</template>
