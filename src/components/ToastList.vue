<script setup lang="ts">
import { useToast } from "../composables/useToast";

const { toasts } = useToast();
</script>

<template>
    <Teleport to="body">
        <div
            class="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 items-end"
        >
            <TransitionGroup
                enter-active-class="transition-all duration-200"
                enter-from-class="opacity-0 translate-y-2"
                leave-active-class="transition-all duration-150"
                leave-to-class="opacity-0 translate-y-2"
            >
                <div
                    v-for="toast in toasts"
                    :key="toast.id"
                    class="flex items-center gap-2.5 px-4 py-2.5 rounded-lg border text-sm shadow-lg"
                    :class="
                        toast.type === 'success'
                            ? 'bg-zinc-900 border-emerald-800 text-emerald-300'
                            : 'bg-zinc-900 border-red-800 text-red-300'
                    "
                >
                    <svg
                        v-if="toast.type === 'success'"
                        class="w-4 h-4 shrink-0"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="m4.5 12.75 6 6 9-13.5"
                        />
                    </svg>
                    <svg
                        v-else
                        class="w-4 h-4 shrink-0"
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
                    {{ toast.message }}
                </div>
            </TransitionGroup>
        </div>
    </Teleport>
</template>
