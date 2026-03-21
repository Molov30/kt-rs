<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import type { Command, CommandGroup } from "../types/commands";
import CommandConfig from "./CommandConfig.vue";
import CommandIcon from "./CommandIcon.vue";

const props = defineProps<{
    groups: CommandGroup[];
    open: boolean;
}>();

const emit = defineEmits<{
    "update:open": [value: boolean];
}>();

const query = ref("");
const activeIndex = ref(0);
const searchInput = ref<HTMLInputElement | null>(null);
const configCommand = ref<Command | null>(null);

const flatFiltered = computed(() => {
    const q = query.value.toLowerCase().trim();
    const result: { group: CommandGroup; command: Command }[] = [];
    for (const group of props.groups) {
        for (const command of group.commands) {
            if (
                !q ||
                command.label.toLowerCase().includes(q) ||
                command.description?.toLowerCase().includes(q)
            ) {
                result.push({ group, command });
            }
        }
    }
    return result;
});

const filteredGroups = computed(() => {
    const q = query.value.toLowerCase().trim();
    return props.groups
        .map((group) => ({
            ...group,
            commands: group.commands.filter(
                (cmd) =>
                    !q ||
                    cmd.label.toLowerCase().includes(q) ||
                    cmd.description?.toLowerCase().includes(q),
            ),
        }))
        .filter((g) => g.commands.length > 0);
});

watch(
    () => props.open,
    async (val) => {
        if (val) {
            query.value = "";
            activeIndex.value = 0;
            configCommand.value = null;
            await nextTick();
            searchInput.value?.focus();
        }
    },
);

watch(query, () => {
    activeIndex.value = 0;
});

function close() {
    emit("update:open", false);
}

function indexOfCommand(cmd: Command) {
    return flatFiltered.value.findIndex((e) => e.command.id === cmd.id);
}

function isActive(cmd: Command) {
    return indexOfCommand(cmd) === activeIndex.value;
}

function selectCommand(cmd: Command) {
    if (cmd.params?.length) {
        configCommand.value = cmd;
    } else {
        cmd.action();
        close();
    }
}

function runWithParams(params: Record<string, string>) {
    configCommand.value?.action(params);
    configCommand.value = null;
    close();
}

function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
        if (configCommand.value) {
            configCommand.value = null;
        } else {
            close();
        }
        return;
    }
    if (configCommand.value) return;

    if (e.key === "ArrowDown") {
        e.preventDefault();
        activeIndex.value = Math.min(
            activeIndex.value + 1,
            flatFiltered.value.length - 1,
        );
    } else if (e.key === "ArrowUp") {
        e.preventDefault();
        activeIndex.value = Math.max(activeIndex.value - 1, 0);
    } else if (e.key === "Enter") {
        const entry = flatFiltered.value[activeIndex.value];
        if (entry) selectCommand(entry.command);
    }
}
</script>

<template>
    <Teleport to="body">
        <Transition
            enter-active-class="transition-opacity duration-150"
            enter-from-class="opacity-0"
            leave-active-class="transition-opacity duration-100"
            leave-to-class="opacity-0"
        >
            <div
                v-if="open"
                class="fixed inset-0 z-50 flex items-start justify-center pt-[6vh]"
                @click.self="close"
                @keydown="onKeydown"
            >
                <!-- backdrop -->
                <div class="absolute inset-0 bg-black/50" @click="close" />

                <!-- palette -->
                <div
                    class="relative w-full max-w-sm rounded-xl bg-zinc-900 border border-zinc-800 shadow-2xl overflow-hidden"
                >
                    <!-- config view -->
                    <div v-if="configCommand" class="p-4">
                        <CommandConfig
                            :command="configCommand"
                            @run="runWithParams"
                            @cancel="configCommand = null"
                        />
                    </div>

                    <!-- search + list view -->
                    <template v-else>
                        <!-- search input -->
                        <div
                            class="flex items-center gap-2 px-3 py-2 border-b border-zinc-800"
                        >
                            <svg
                                class="w-3.5 h-3.5 text-zinc-500 shrink-0"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                viewBox="0 0 24 24"
                            >
                                <circle cx="11" cy="11" r="8" />
                                <path d="m21 21-4.35-4.35" />
                            </svg>
                            <input
                                ref="searchInput"
                                v-model="query"
                                type="text"
                                placeholder="Search..."
                                class="flex-1 bg-transparent text-zinc-100 text-xs placeholder:text-zinc-600 focus:outline-none"
                                @keydown="onKeydown"
                            />
                            <kbd
                                class="text-[9px] text-zinc-600 border border-zinc-700 rounded px-1 py-0.5"
                                >ESC</kbd
                            >
                        </div>

                        <!-- command groups -->
                        <div class="max-h-64 overflow-y-auto py-1">
                            <template v-if="filteredGroups.length === 0">
                                <p
                                    class="text-xs text-zinc-600 text-center py-6"
                                >
                                    No results for "{{ query }}"
                                </p>
                            </template>

                            <template
                                v-for="group in filteredGroups"
                                :key="group.id"
                            >
                                <div class="px-3 pt-2 pb-0.5">
                                    <span
                                        class="text-[9px] font-semibold text-zinc-600 uppercase tracking-widest"
                                        >{{ group.label }}</span
                                    >
                                </div>

                                <button
                                    v-for="cmd in group.commands"
                                    :key="cmd.id"
                                    class="w-full flex items-center gap-2 px-3 py-1.5 text-left transition-colors"
                                    :class="
                                        isActive(cmd)
                                            ? 'bg-zinc-800'
                                            : 'hover:bg-zinc-800/50'
                                    "
                                    @click="selectCommand(cmd)"
                                    @mouseenter="
                                        activeIndex = indexOfCommand(cmd)
                                    "
                                >
                                    <CommandIcon
                                        v-if="cmd.icon"
                                        :icon="cmd.icon"
                                    />
                                    <div
                                        v-else
                                        class="w-3.5 h-3.5 shrink-0 rounded bg-zinc-800 border border-zinc-700 flex items-center justify-center"
                                    >
                                        <span
                                            class="text-[8px] text-zinc-600 font-mono leading-none"
                                        >
                                            {{
                                                cmd.label
                                                    .charAt(0)
                                                    .toUpperCase()
                                            }}
                                        </span>
                                    </div>

                                    <span class="text-xs text-zinc-200 truncate flex-1">{{ cmd.label }}</span>

                                    <div
                                        class="flex items-center gap-1.5 shrink-0"
                                    >
                                        <span
                                            v-if="cmd.params?.length"
                                            class="text-[9px] text-zinc-600 border border-zinc-700 rounded px-1 py-0.5"
                                            >cfg</span
                                        >
                                        <kbd
                                            v-if="cmd.shortcut"
                                            class="text-[9px] text-zinc-500 border border-zinc-700 rounded px-1 py-0.5"
                                            >{{ cmd.shortcut }}</kbd
                                        >
                                    </div>
                                </button>

                                <div
                                    class="mx-3 my-0.5 border-b border-zinc-800/60 last:hidden"
                                />
                            </template>
                        </div>

                        <!-- footer -->
                        <div
                            class="flex items-center gap-2 px-3 py-1.5 border-t border-zinc-800 text-[9px] text-zinc-600"
                        >
                            <span><kbd class="border border-zinc-700 rounded px-1">↑↓</kbd> nav</span>
                            <span><kbd class="border border-zinc-700 rounded px-1">↵</kbd> run</span>
                            <span><kbd class="border border-zinc-700 rounded px-1">ESC</kbd> close</span>
                        </div>
                    </template>
                </div>
            </div>
        </Transition>
    </Teleport>
</template>
