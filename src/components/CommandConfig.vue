<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import type { Command } from "../types/commands";
import CommandIcon from "./CommandIcon.vue";

const props = defineProps<{
    command: Command;
}>();

const emit = defineEmits<{
    run: [params: Record<string, string>];
    cancel: [];
}>();

const values = ref<Record<string, string>>({});

onMounted(() => {
    for (const param of props.command.params ?? []) {
        values.value[param.name] = param.default ?? "";
    }
});

const isValid = computed(() => {
    return (props.command.params ?? []).every((p) => {
        if (p.required === false) return true;
        return values.value[p.name]?.trim() !== "";
    });
});

function submit() {
    if (!isValid.value) return;
    emit("run", { ...values.value });
}
</script>

<template>
    <div class="flex flex-col gap-4">
        <div class="flex items-center gap-2 border-b border-zinc-800 pb-3">
            <button
                class="text-zinc-500 hover:text-zinc-300 transition-colors text-xs"
                @click="emit('cancel')"
            >
                ← back
            </button>
            <CommandIcon v-if="command.icon" :icon="command.icon" />
            <span class="text-zinc-200 text-sm font-medium">{{
                command.label
            }}</span>
        </div>

        <div class="flex flex-col gap-3">
            <div
                v-for="param in command.params"
                :key="param.name"
                class="flex flex-col gap-1"
            >
                <label
                    class="text-xs text-zinc-500 font-medium uppercase tracking-wider"
                >
                    {{ param.label }}
                </label>

                <select
                    v-if="param.type === 'select'"
                    v-model="values[param.name]"
                    class="bg-zinc-800 border border-zinc-700 rounded text-zinc-200 text-sm px-3 py-2 focus:outline-none focus:border-zinc-500 transition-colors"
                >
                    <option
                        v-for="opt in param.options"
                        :key="opt"
                        :value="opt"
                    >
                        {{ opt }}
                    </option>
                </select>

                <input
                    v-else
                    v-model="values[param.name]"
                    :type="param.type === 'number' ? 'number' : 'text'"
                    :placeholder="param.placeholder"
                    class="bg-zinc-800 border border-zinc-700 rounded text-zinc-200 text-sm px-3 py-2 focus:outline-none focus:border-zinc-500 transition-colors placeholder:text-zinc-600"
                    @keydown.enter="submit"
                    @keydown.escape="emit('cancel')"
                />
            </div>
        </div>

        <div class="flex justify-end gap-2 pt-1">
            <button
                class="text-xs text-zinc-500 hover:text-zinc-300 transition-colors px-3 py-1.5"
                @click="emit('cancel')"
            >
                Cancel
            </button>
            <button
                :disabled="!isValid"
                class="text-xs bg-zinc-100 text-zinc-900 rounded px-3 py-1.5 font-medium hover:bg-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                @click="submit"
            >
                Run
            </button>
        </div>
    </div>
</template>
