<script setup lang="ts">
import { Monitor, RefreshCw } from '@lucide/vue'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import StatusPill from '@/components/common/StatusPill.vue'
import { useAppStore } from '@/stores/app.store'

const route = useRoute()
const appStore = useAppStore()

const title = computed(() => route.meta.title ?? 'money_clock')
const platformLabel = computed(() => appStore.platformInfo?.displayName ?? '未连接桌面端')
const isRefreshing = computed(() => appStore.environmentStatus === 'loading')

async function refresh(): Promise<void> {
  await appStore.loadEnvironment(true)
}
</script>

<template>
  <header
    class="flex h-16 shrink-0 items-center justify-between gap-4 border-b border-white/10 bg-ink-950/80 px-6 backdrop-blur"
  >
    <div class="min-w-0">
      <p class="truncate text-sm font-semibold text-white">{{ title }}</p>
      <p class="truncate text-xs text-slate-500">money_clock · 模块化桌面应用基础架构</p>
    </div>

    <div class="flex items-center gap-3">
      <StatusPill
        :label="appStore.isDesktop ? 'Tauri IPC' : '浏览器预览'"
        :tone="appStore.isDesktop ? 'success' : 'warning'"
      />
      <div
        class="hidden items-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-1.5 text-xs text-slate-300 sm:flex"
      >
        <Monitor class="h-3.5 w-3.5 text-slate-400" />
        <span>{{ platformLabel }}</span>
      </div>
      <button
        type="button"
        class="grid h-8 w-8 place-items-center rounded-lg border border-white/10 text-slate-400 transition hover:bg-white/5 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="!appStore.isDesktop || isRefreshing"
        title="重新检测运行环境"
        @click="refresh"
      >
        <RefreshCw class="h-4 w-4" :class="isRefreshing ? 'animate-spin' : ''" />
      </button>
    </div>
  </header>
</template>
