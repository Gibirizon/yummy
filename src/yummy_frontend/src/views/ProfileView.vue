<script setup>
import { useRouter, useRoute } from "vue-router";
import { onBeforeMount, watch } from "vue";
import { useAuthStore } from "./../store/auth";
import { storeToRefs } from "pinia";
import { retryICCall } from "../retry/icRetry";

const router = useRouter();
const route = useRoute();
const { retryCall } = retryICCall();
const authStore = useAuthStore();
const { isAuthenticated, whoamiActor } = storeToRefs(authStore);
const id_route = BigInt(route.params.id);

async function getUserIndex() {
    if (!isAuthenticated.value) {
        goToHome();
        return;
    }
    let user_index = await retryCall(() => whoamiActor.value.get_user_index());
    if (!user_index || !user_index.Ok || user_index.Ok !== id_route) {
        goToHome();
        return;
    }
}
function goToHome() {
    router.push({ name: "home", query: { canisterId: route.query.canisterId } });
}

onBeforeMount(async () => {
    setTimeout(async () => {
        await getUserIndex();
    }, 100);
});
</script>

<template>
    <RouterView @go-to-home="goToHome" />
</template>
