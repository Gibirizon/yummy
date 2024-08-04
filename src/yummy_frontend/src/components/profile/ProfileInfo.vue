<script setup>
import { ref, onBeforeMount } from "vue";
import { yummy_backend } from "declarations/yummy_backend/index";
import { useRouter, useRoute } from "vue-router";

const emit = defineEmits(["go-to-home"]);
const router = useRouter();
const route = useRoute();

// user
const username = ref(null);
const id = ref(null);

// id
const id_from_route = BigInt(route.params.id);

async function getUser() {
    const user = await yummy_backend.get_user_by_index(id_from_route);
    if (!user.Ok) {
        emit("go-to-home");
    }
    username.value = user.Ok.name;
    id.value = user.Ok.id;
}
function goToEdit() {
    router.push({
        name: "profile-edit",
        query: { canisterId: route.query.canisterId },
        params: { id: id_from_route },
    });
}

onBeforeMount(async () => {
    await getUser();
});
</script>
<template>
    <div class="profile-info flex min-h-screen w-full flex-col items-center justify-center bg-gray-900 p-8 text-white">
        <div class="w-[90%] max-w-2xl">
            <h1 class="mb-8 text-center text-4xl font-bold">User Profile</h1>
            <div class="flex flex-col gap-6 rounded-lg bg-gray-800 px-8 py-10 shadow-lg">
                <div class="flex items-center">
                    <p class="mr-2 text-2xl font-semibold">Identifier:</p>
                    <span>{{ id }}</span>
                </div>
                <p class="text-2xl"><span class="mr-2 font-semibold">Username:</span> {{ username }}</p>
                <button
                    @click="goToEdit"
                    class="w-full transform rounded-lg bg-blue-600 px-6 py-4 text-xl font-bold text-white transition duration-300 ease-in-out hover:scale-105 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
                >
                    Edit Profile
                </button>
            </div>
        </div>
    </div>
</template>
