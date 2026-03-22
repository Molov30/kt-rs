<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

defineProps<{
    dumps: string[];
}>();

const emit = defineEmits<{
    dismiss: [];
}>();

let intervalId: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
    invoke("play_alert");
    intervalId = setInterval(() => invoke("play_alert"), 4000);
});

onUnmounted(() => {
    if (intervalId) clearInterval(intervalId);
});
</script>

<template>
    <div class="fixed inset-x-0 top-0 z-50 flex justify-center p-3">
        <div
            class="w-full max-w-sm bg-red-950 border border-red-700 rounded-lg px-4 py-3 shadow-xl"
        >
            <div class="flex items-start gap-3">
                <svg
                    class="w-4 h-4 text-red-400 shrink-0 mt-0.5"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z"
                    />
                </svg>
                <div class="flex-1 min-w-0">
                    <p class="text-xs font-semibold text-red-300 mb-1">
                        New dump{{ dumps.length > 1 ? `s (${dumps.length})` : "" }}
                    </p>
                    <ul class="space-y-0.5">
                        <li
                            v-for="d in dumps"
                            :key="d"
                            class="text-[10px] text-red-400 font-mono truncate"
                        >
                            {{ d }}
                        </li>
                    </ul>
                </div>
                <button
                    class="shrink-0 text-[10px] font-medium text-red-300 border border-red-700 rounded px-2 py-1 hover:bg-red-900 transition-colors"
                    @click="emit('dismiss')"
                >
                    Mark as read
                </button>
            </div>
        </div>
    </div>
</template>
