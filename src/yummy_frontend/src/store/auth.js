import { defineStore } from "pinia";
import { AuthClient } from "@dfinity/auth-client";
import { ref } from "vue";
import { createActor, canisterId } from "declarations/yummy_backend";

export const getIdentityProvider = () => {
    let idpProvider;
    // Safeguard against server rendering
    if (typeof window !== "undefined") {
        const isLocal = process.env.DFX_NETWORK !== "ic";
        // Safari does not support localhost subdomains
        const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
        if (isLocal && isSafari) {
            idpProvider = `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
        } else if (isLocal) {
            idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
        }
    }
    return idpProvider;
};

export const defaultOptions = {
    createOptions: {
        idleOptions: {
            disableIdle: true,
        },
    },
    loginOptions: {
        identityProvider: getIdentityProvider(),
    },
};

function actorFromIdentity(identity) {
    return createActor(canisterId, {
        agentOptions: {
            identity,
        },
    });
}

export const useAuthStore = defineStore("auth", () => {
    const isReady = ref(false);
    const isAuthenticated = ref(null);
    const authClient = ref(null);
    const identity = ref(null);
    const whoamiActor = ref(null);

    const init = async () => {
        authClient.value = await AuthClient.create(defaultOptions.createOptions);
        isAuthenticated.value = await authClient.value.isAuthenticated();
        identity.value = isAuthenticated.value ? authClient.value.getIdentity() : null;
        whoamiActor.value = identity.value ? actorFromIdentity(identity.value) : null;
        isReady.value = true;
    };

    const login = async () => {
        return new Promise((resolve, reject) => {
            authClient.value.login({
                ...defaultOptions.loginOptions,
                identityProvider: getIdentityProvider(),
                maxTimeToLive: BigInt(7) * BigInt(24) * BigInt(3_600_000_000_000),
                onSuccess: async () => {
                    try {
                        isAuthenticated.value = await authClient.value.isAuthenticated();
                        identity.value = isAuthenticated.value ? authClient.value.getIdentity() : null;
                        whoamiActor.value = identity.value ? actorFromIdentity(identity.value) : null;
                        console.log("Login Success");
                        resolve();
                    } catch (error) {
                        console.error("Error during post-login process:", error);
                        reject(error);
                    }
                },
                onError: (error) => {
                    console.error("Login Failed: ", error);
                    reject(error);
                },
            });
        });
    };

    const logout = async () => {
        await authClient.value?.logout();
        isAuthenticated.value = false;
        identity.value = null;
        whoamiActor.value = null;
    };

    return {
        isReady,
        isAuthenticated,
        authClient,
        identity,
        whoamiActor,
        init,
        login,
        logout,
    };
});
