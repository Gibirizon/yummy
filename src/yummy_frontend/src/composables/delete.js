// useDeleteRecipe.js
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "../store/auth";
import { useMessageStore } from "../store/message";

export function useDelete() {
    const authStore = useAuthStore();
    const { isAuthenticated } = storeToRefs(authStore);
    const messageStore = useMessageStore();

    const showDeleteConfirmation = ref(false);

    async function confirmDelete(backend_function) {
        messageStore.showMessage("Deleting...", "warning");
        showDeleteConfirmation.value = false;

        if (!isAuthenticated.value) {
            messageStore.showMessage("Not logged in", "error");
            return;
        }

        let deleteResponse = await backend_function();
        if (deleteResponse.Err) {
            responseKey = Object.keys(deleteResponse.Err)[0];
            messageStore.showMessage(deleteResponse.Err.responseKey.msg, "warning");
            return;
        }

        messageStore.showMessage(deleteResponse.Ok, "success");
    }

    return {
        showDeleteConfirmation,
        confirmDelete,
    };
}
