<script setup>
let url = "/images/header_salad.png";
</script>
<template>
    <header class="relative w-full">
        <div class="h-[400px] w-full bg-cover bg-center" :style="{ backgroundImage: `url(${url})` }">
            <div class="h-full w-full bg-black/70"></div>
            <div class="absolute left-[50%] top-[50%] w-full translate-x-[-50%] translate-y-[-50%] text-center">
                <h1
                    class="mb-[10px] font-['Dancing_Script'] text-[48px] font-bold uppercase tracking-[2px] text-[#ddd]"
                >
                    Y<span class="lowercase text-[#09f]">umm</span>y
                </h1>
                <p class="text-[#ddd]">Find and share recipes</p>
            </div>
        </div>
    </header>
</template>

<style scoped>
@media screen and (min-width: 1200px) {
    h1 {
        @apply text-[64px];
    }
    p {
        @apply text-[20px];
    }
}
</style>
