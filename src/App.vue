<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import CommandPalette from "./components/CommandPalette.vue";
import CommandIcon from "./components/CommandIcon.vue";
import CommandConfig from "./components/CommandConfig.vue";
import ToastList from "./components/ToastList.vue";
import type { Command, CommandGroup } from "./types/commands";
import { useToast } from "./composables/useToast";

const paletteOpen = ref(false);
const lastResult = ref("");
const configCommand = ref<Command | null>(null);
const { show } = useToast();

const icons = {
    greet: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 0 1 .865-.501 48.172 48.172 0 0 0 3.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0 0 12 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018Z"/></svg>`,
    clear: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"/></svg>`,
    search: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>`,
    start: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347c-.75.412-1.667-.13-1.667-.986V5.653Z"/></svg>`,
    stop: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M5.25 7.5A2.25 2.25 0 0 1 7.5 5.25h9a2.25 2.25 0 0 1 2.25 2.25v9a2.25 2.25 0 0 1-2.25 2.25h-9a2.25 2.25 0 0 1-2.25-2.25v-9Z"/></svg>`,
    restart: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/></svg>`,
    debug: `<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0 1 12 12.75Zm0 0c2.883 0 5.647.508 8.207 1.44a23.91 23.91 0 0 1-1.152 6.06M12 12.75c-2.883 0-5.647.508-8.208 1.44a23.916 23.916 0 0 0 1.152 6.061M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0 1 12 12.75ZM12 3v9.75m0-9.75a3 3 0 0 0-3 3m3-3a3 3 0 0 1 3 3m-6 0h6m-6 0a3 3 0 0 0-3 3v1.5m12-4.5a3 3 0 0 1 3 3v1.5m0 0a3 3 0 0 1-3 3h-.75M3.75 9.75a3 3 0 0 0-3 3v1.5m0 0a3 3 0 0 0 3 3h.75"/></svg>`,
};

async function runServiceCommand(command: string, label: string) {
    try {
        await invoke<string>(command);
        show(`${label}: OK`, "success");
    } catch (e) {
        show(`${label}: ${e}`, "error");
    }
}

async function notifyDump(path: string) {
    await invoke("send_notification", { title: "New dump", body: path });
}

const groups: CommandGroup[] = [
    {
        id: "service",
        label: "Service",
        commands: [
            {
                id: "start-service",
                label: "Start service",
                description: "Start the axxon-next systemd service",
                icon: icons.start,
                action: () =>
                    runServiceCommand("start_service", "Start service"),
            },
            {
                id: "stop-service",
                label: "Stop service",
                description: "Stop the axxon-next systemd service",
                icon: icons.stop,
                action: () => runServiceCommand("stop_service", "Stop service"),
            },
            {
                id: "restart-service",
                label: "Restart service",
                description: "Restart the axxon-next systemd service",
                icon: icons.restart,
                action: () =>
                    runServiceCommand("restart_service", "Restart service"),
            },
            {
                id: "kill-ui",
                label: "Kill UI",
                description: "Kill axxon-next UI",
                icon: icons.stop,
                action: () => runServiceCommand("kill_ui", "Kill UI"),
            },
            {
                id: "enable-debug",
                label: "Enable debug logging",
                description: "Replace INFO with DEBUG in instance.conf",
                icon: icons.debug,
                action: () => runServiceCommand("enable_debug", "Enable debug"),
            },
            {
                id: "disable-obsolete",
                label: "Disable obsolete",
                description: "Set obsolete=false in all config files",
                icon: icons.clear,
                action: () =>
                    runServiceCommand("disable_obsolete", "Disable obsolete"),
            },
            {
                id: "clear-logs",
                label: "Clear logs",
                description: "Delete all log files",
                icon: icons.clear,
                action: () => runServiceCommand("clear_logs", "Clear logs"),
            },
        ],
    },
    {
        id: "view",
        label: "View",
        commands: [
            {
                id: "toggle-palette",
                label: "Open command palette",
                icon: icons.search,
                shortcut: "⌘K",
                action: () => {
                    paletteOpen.value = true;
                },
            },
        ],
    },
];

function selectInline(cmd: Command) {
    if (cmd.params?.length) {
        configCommand.value = cmd;
    } else {
        cmd.action();
    }
}

function runWithParams(params: Record<string, string>) {
    configCommand.value?.action(params);
    configCommand.value = null;
}

function onKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        paletteOpen.value = !paletteOpen.value;
    }
}

onMounted(async () => {
    window.addEventListener("keydown", onKeydown);
    await listen<string>("new-dump", (event) => {
        show(`New dump: ${event.payload}`, "error");
        notifyDump(event.payload);
    });
});
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
    <div
        class="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col items-center pt-8 px-4 font-sans"
    >
        <!-- search bar -->
        <div class="w-full max-w-sm">
            <button
                class="w-full flex items-center gap-2 px-3 py-2 rounded-lg border border-zinc-800 bg-zinc-900 text-zinc-500 text-xs hover:border-zinc-700 hover:text-zinc-300 transition-colors"
                @click="paletteOpen = true"
            >
                <svg
                    class="w-3.5 h-3.5 shrink-0"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    viewBox="0 0 24 24"
                >
                    <circle cx="11" cy="11" r="8" />
                    <path d="m21 21-4.35-4.35" />
                </svg>
                <span class="flex-1 text-left">Search commands...</span>
                <kbd class="text-[9px] border border-zinc-700 rounded px-1"
                    >⌘K</kbd
                >
            </button>
        </div>

        <!-- result -->
        <div class="w-full max-w-sm mt-3" v-if="lastResult">
            <p
                class="text-xs text-zinc-400 bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2"
            >
                {{ lastResult }}
            </p>
        </div>

        <!-- inline config -->
        <div
            v-if="configCommand"
            class="w-full max-w-sm mt-5 rounded-lg border border-zinc-800 bg-zinc-900 px-4 py-4"
        >
            <CommandConfig
                :command="configCommand"
                @run="runWithParams"
                @cancel="configCommand = null"
            />
        </div>

        <!-- command groups -->
        <div v-else class="w-full max-w-sm mt-5 flex flex-col gap-4">
            <div v-for="group in groups" :key="group.id">
                <div class="mb-1 px-1">
                    <span
                        class="text-[9px] font-semibold text-zinc-600 uppercase tracking-widest"
                        >{{ group.label }}</span
                    >
                </div>
                <div class="rounded-lg border border-zinc-800 overflow-hidden">
                    <button
                        v-for="(cmd, i) in group.commands"
                        :key="cmd.id"
                        class="w-full flex items-center gap-2.5 px-3 py-2 text-left transition-colors hover:bg-zinc-800/60 active:bg-zinc-800"
                        :class="
                            i < group.commands.length - 1
                                ? 'border-b border-zinc-800'
                                : ''
                        "
                        @click="selectInline(cmd)"
                    >
                        <CommandIcon v-if="cmd.icon" :icon="cmd.icon" />
                        <div
                            v-else
                            class="w-3.5 h-3.5 shrink-0 rounded bg-zinc-800 border border-zinc-700 flex items-center justify-center"
                        >
                            <span
                                class="text-[8px] text-zinc-600 font-mono leading-none"
                                >{{ cmd.label.charAt(0).toUpperCase() }}</span
                            >
                        </div>
                        <span class="text-xs text-zinc-200 flex-1 truncate">{{
                            cmd.label
                        }}</span>
                        <span
                            v-if="cmd.description"
                            class="text-[10px] text-zinc-600 truncate max-w-[120px] hidden sm:block"
                            >{{ cmd.description }}</span
                        >
                        <kbd
                            v-if="cmd.shortcut"
                            class="text-[9px] text-zinc-500 border border-zinc-700 rounded px-1 py-0.5 shrink-0"
                            >{{ cmd.shortcut }}</kbd
                        >
                        <svg
                            v-if="cmd.params?.length"
                            class="w-3 h-3 text-zinc-600 shrink-0"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="m9 18 6-6-6-6"
                            />
                        </svg>
                    </button>
                </div>
            </div>
        </div>

        <CommandPalette v-model:open="paletteOpen" :groups="groups" />
        <ToastList />
    </div>
</template>
