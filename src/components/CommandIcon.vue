<script setup lang="ts">
import type { Component } from 'vue'

defineProps<{
  icon: string | Component
}>()
</script>

<template>
  <span
    v-if="typeof icon === 'string'"
    class="w-4 h-4 shrink-0 text-zinc-400 flex items-center justify-center [&>svg]:w-4 [&>svg]:h-4 [&>svg]:shrink-0"
    v-html="icon"
  />
  <component
    v-else
    :is="icon"
    class="w-4 h-4 shrink-0 text-zinc-400"
  />
</template>
