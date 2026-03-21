<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import CommandPalette from './components/CommandPalette.vue'
import type { CommandGroup } from './types/commands'

const paletteOpen = ref(false)
const lastResult = ref('')

const icons = {
  greet: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 0 1 .865-.501 48.172 48.172 0 0 0 3.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0 0 12 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018Z"/></svg>`,
  clear: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"/></svg>`,
  search: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>`,
}

const groups: CommandGroup[] = [
  {
    id: 'system',
    label: 'System',
    commands: [
      {
        id: 'greet',
        label: 'Greet from Rust',
        description: 'Call the Tauri greet command',
        icon: icons.greet,
        params: [
          { name: 'name', label: 'Your name', type: 'text', placeholder: 'World', default: '', required: true },
        ],
        action: async (params) => {
          const result = await invoke<string>('greet', { name: params?.name ?? 'World' })
          lastResult.value = result
        },
      },
      {
        id: 'clear',
        label: 'Clear output',
        description: 'Clear the result area',
        icon: icons.clear,
        action: () => {
          lastResult.value = ''
        },
      },
    ],
  },
  {
    id: 'view',
    label: 'View',
    commands: [
      {
        id: 'toggle-palette',
        label: 'Open command palette',
        icon: icons.search,
        shortcut: '⌘K',
        action: () => {
          paletteOpen.value = true
        },
      },
    ],
  },
]

function onKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    paletteOpen.value = !paletteOpen.value
  }
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <div class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col items-center justify-center gap-6 font-sans">
    <div class="text-center space-y-2">
      <h1 class="text-2xl font-semibold tracking-tight">Command Palette</h1>
      <p class="text-sm text-zinc-500">Press <kbd class="border border-zinc-700 rounded px-1.5 py-0.5 text-zinc-400">⌘K</kbd> to open</p>
    </div>

    <button
      class="flex items-center gap-2 px-4 py-2 rounded-lg border border-zinc-800 bg-zinc-900 text-zinc-400 text-sm hover:border-zinc-700 hover:text-zinc-200 transition-colors"
      @click="paletteOpen = true"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
      </svg>
      Search commands...
      <kbd class="ml-2 text-[10px] border border-zinc-700 rounded px-1">⌘K</kbd>
    </button>

    <p v-if="lastResult" class="text-sm text-zinc-400 bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-3 max-w-sm text-center">
      {{ lastResult }}
    </p>

    <CommandPalette v-model:open="paletteOpen" :groups="groups" />
  </div>
</template>
