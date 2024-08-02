<script setup>
import { useRouter, useRoute } from "vue-router";
import { useAuthStore } from "./../store/auth";

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();
const id_route = BigInt(route.params.id);

async function getUserIndex() {
    if (!authStore.whoamiActor) {
        goToHome();
    }
    let user_index = await authStore.whoamiActor?.get_user_index();
    if (!user_index.Ok || user_index.Ok !== id_route) {
        goToHome();
    }
    console.log("User index is correct: ", user_index.Ok);
}
getUserIndex();
function goToHome() {
    router.push({ name: "home", query: { canisterId: route.query.canisterId } });
}
</script>

<template>
    <RouterView @go-to-home="goToHome" />
</template>
