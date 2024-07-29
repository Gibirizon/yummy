<script setup>
import { useRouter, useRoute } from "vue-router";
import { useAuthStore } from "./../store/auth";

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();
const id_route = BigInt(route.params.id);

async function getUserIndex() {
    console.log("get user index");
    if (authStore.whoamiActor) {
        await authStore.whoamiActor?.get_user_index_by_principal().then(async (index) => {
            if (index.Ok && index.Ok === id_route) {
                console.log("User index is correct: ", index.Ok);
            } else {
                goToHome();
            }
        });
    }
}
getUserIndex();
function goToHome() {
    router.push({ name: "home", query: { canisterId: route.query.canisterId } });
}
</script>

<template>
    <RouterView @go-to-home="goToHome" />
</template>
