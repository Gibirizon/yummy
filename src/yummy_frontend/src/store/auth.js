import { defineStore } from "pinia";
import { AuthClient } from "@dfinity/auth-client";
import { toRaw } from "vue";
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

export const useAuthStore = () => {
    const innerStore = defineStore("auth", {
        id: "auth",
        state: () => {
            return {
                initialisation: true,
                isReady: false,
                isAuthenticated: null,
                authClient: null,
                identity: null,
                whoamiActor: null,
            };
        },
        actions: {
            async init() {
                return new Promise(async (resolve, reject) => {
                    try {
                        const authClient = await AuthClient.create(defaultOptions.createOptions);
                        this.authClient = authClient;
                        const isAuthenticated = await authClient.isAuthenticated();
                        const identity = isAuthenticated ? authClient.getIdentity() : null;
                        const whoamiActor = identity ? actorFromIdentity(identity) : null;
                        this.initialisation = false;
                        this.isAuthenticated = isAuthenticated;
                        this.identity = identity;
                        this.whoamiActor = whoamiActor;
                        this.isReady = true;
                        resolve();
                    } catch (error) {
                        console.error("Error during initialization:", error);
                        reject(error);
                    }
                });
            },
            async login() {
                const authClient = toRaw(this.authClient);
                return new Promise((resolve, reject) => {
                    authClient.login({
                        ...defaultOptions.loginOptions,
                        identityProvider: getIdentityProvider(),
                        maxTimeToLive: BigInt(7) * BigInt(24) * BigInt(3_600_000_000_000),
                        onSuccess: async () => {
                            try {
                                this.isAuthenticated = await authClient.isAuthenticated();
                                this.identity = this.isAuthenticated ? authClient.getIdentity() : null;
                                this.whoamiActor = this.identity ? actorFromIdentity(this.identity) : null;
                                console.log("Login Successful");
                                resolve();
                            } catch (error) {
                                console.error("Error during post-login process:", error);
                                reject(error);
                            }
                        },
                        onError: (error) => {
                            console.error("Login Failed: ", error);
                        },
                    });
                });
            },
            async logout() {
                const authClient = toRaw(this.authClient);
                await authClient?.logout();
                this.isAuthenticated = false;
                this.identity = null;
                this.whoamiActor = null;
            },
        },
    });
    const s = innerStore();
    if (s.initialisation == true) {
        s.init().then(() => {
            return s;
        });
    }
    return s;
};
