/**
 * Retries an Internet Computer API call with exponential backoff.
 * @param {Function} apiCall - The async function to call the IC API.
 * @param {number} [maxRetries=3] - Maximum number of retry attempts.
 * @param {number} [initialDelay=100] - Initial delay in milliseconds before the first retry.
 * @returns {Promise<any>} - The result of the successful API call.
 * @throws {Error} - Throws an error if all retries fail.
 */
import { useAuthStore } from "../store/auth";
import { storeToRefs } from "pinia";
export function retryICCall() {
    const authStore = useAuthStore();
    const { isAuthenticated } = storeToRefs(authStore);

    async function retryCall(apiCall, maxRetries = 3, initialDelay = 100) {
        let retries = 0;
        while (retries < maxRetries) {
            try {
                if (retries >= maxRetries && !isAuthenticated.value) {
                    authStore.logout();
                    return;
                }
                return await apiCall();
            } catch (error) {
                if (error.message.includes("Invalid certificate: Signature verification failed")) {
                    retries++;
                    if (retries >= maxRetries) throw error;
                    console.log(
                        `Retry authentication attempt ${retries} after ${initialDelay * Math.pow(2, retries - 1)}ms`
                    );
                    await new Promise((resolve) => setTimeout(resolve, initialDelay * Math.pow(2, retries - 1)));
                } else {
                    console.log(error);
                    // throw error; // If it's a different error, throw immediately
                }
            }
        }
    }

    return {
        retryCall,
    };
}
