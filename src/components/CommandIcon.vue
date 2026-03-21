<script setup lang="ts">
import type { Component } from "vue";

defineProps<{
    icon: string | Component;
}>();
</script>

<template>
    <span
        v-if="typeof icon === 'string'"
        class="w-3.5 h-3.5 shrink-0 text-zinc-400 flex items-center justify-center [&>svg]:w-3.5 [&>svg]:h-3.5 [&>svg]:shrink-0"
        v-html="icon"
    />
    <component v-else :is="icon" class="w-3.5 h-3.5 shrink-0 text-zinc-400" />
</template>
