<script setup>
import { UtensilsCrossed, ChefHat, Smartphone, Key, Network, CookingPot, Pizza } from "lucide-vue-next";
</script>

<template>
    <div class="flex min-h-screen flex-col bg-gray-800 p-8 text-white">
        <h1 class="mb-4 flex items-center text-4xl font-bold">
            Yummy
            <UtensilsCrossed class="ml-2 h-8 w-8" />
        </h1>

        <p class="mb-8 text-xl">
            Yummy is a modern web application designed for finding and sharing recipes. It leverages the power of the
            Internet Computer (IC), providing a decentralized and scalable solution.
        </p>

        <h2 class="mb-4 text-3xl font-semibold">Features</h2>
        <ul class="mb-8 list-inside list-disc space-y-4 pl-4 text-lg">
            <li><strong>Recipe Sharing:</strong> Discover and share a variety of recipes with the community.</li>
            <li>
                <strong>User-Friendly UI:</strong> Intuitive, responsive, and easy-to-use interface for a seamless user
                experience.
            </li>
            <li>
                <strong>Internet Identity Login:</strong> Securely log in using your Internet Identity, providing a
                seamless and secure authentication experience.
            </li>
            <li>
                <strong>Internet Computer Integration:</strong> Utilizes the Internet Computer network for a scalable,
                decentralized infrastructure.
            </li>
            <li>
                <strong>Open Source</strong>All source code is available on
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/Gibirizon/yummy"
                    class="text-blue-400 hover:underline"
                    >Github</a
                >
            </li>
        </ul>

        <h2 class="mb-4 text-3xl font-semibold">Technologies</h2>
        <ul class="mb-8 list-inside list-disc space-y-4 pl-4 text-lg">
            <li>
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/rust-lang/rust"
                    class="text-blue-400 hover:underline"
                    >Rust 🦀</a
                >
                for backend
            </li>
            <li>
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/vuejs/core"
                    class="text-blue-400 hover:underline"
                    >Vue.js</a
                >
                for frontend
            </li>
            <li>
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/vuejs/pinia"
                    class="text-blue-400 hover:underline"
                    >Pinia 🍍</a
                >
                for state management
            </li>
            <li>
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/tailwindlabs/tailwindcss"
                    class="text-blue-400 hover:underline"
                    >TailwindCSS 🍃</a
                >
                for styling
            </li>
            <li>
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/dfinity/cdk-rs"
                    class="text-blue-400 hover:underline"
                    >DfinityCDK</a
                >
                for compiling to the Internet Computer
            </li>
        </ul>

        <p class="text-2xl font-semibold">
            Enjoy discovering and sharing recipes on Yummy!
            <CookingPot class="ml-2 inline-block h-8 w-8" />
            <Pizza class="ml-2 inline-block h-8 w-8" />
        </p>
    </div>
</template>
