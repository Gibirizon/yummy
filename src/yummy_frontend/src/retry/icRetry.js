/**
 * Retries an Internet Computer API call with exponential backoff.
 * @param {Function} apiCall - The async function to call the IC API.
 * @param {number} [maxRetries=3] - Maximum number of retry attempts.
 * @param {number} [initialDelay=100] - Initial delay in milliseconds before the first retry.
 * @returns {Promise<any>} - The result of the successful API call.
 * @throws {Error} - Throws an error if all retries fail.
 */
export async function retryICCall(apiCall, maxRetries = 3, initialDelay = 100) {
    let retries = 0;
    while (retries < maxRetries) {
        try {
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
