<script setup lang="ts">
import { Cpu, LayoutDashboard } from '@lucide/vue'
import { computed } from 'vue'
import type { Component } from 'vue'
import { RouterLink, useRoute } from 'vue-router'

import AppLogo from '@/components/common/AppLogo.vue'
import StatusPill from '@/components/common/StatusPill.vue'
import { useAppStore } from '@/stores/app.store'

interface NavigationItem {
  label: string
  to: string
  icon: Component
}

const navigation: NavigationItem[] = [
  { label: '概览', to: '/', icon: LayoutDashboard },
  { label: '运行环境', to: '/environment', icon: Cpu },
]

const route = useRoute()
const appStore = useAppStore()

const runtime = computed(() => {
  switch (appStore.environmentStatus) {
    case 'ready':
      return { label: '桌面环境', tone: 'success' as const }
    case 'browser':
      return { label: '浏览器预览', tone: 'warning' as const }
    case 'error':
      return { label: '环境异常', tone: 'danger' as const }
    case 'loading':
      return { label: '正在检测', tone: 'info' as const }
    default:
      return { label: '待检测', tone: 'neutral' as const }
  }
})

function isActive(item: NavigationItem): boolean {
  return item.to === '/' ? route.path === '/' : route.path.startsWith(item.to)
}
</script>

<template>
  <aside class="w-64 shrink-0 flex-col border-r border-white/10 bg-ink-900/60">
    <div class="flex h-16 items-center border-b border-white/10 px-5">
      <AppLogo />
    </div>

    <nav class="flex-1 space-y-1 overflow-y-auto px-3 py-4">
      <p class="px-3 pb-2 text-[10px] font-semibold uppercase tracking-[0.2em] text-slate-600">
        Navigation
      </p>
      <RouterLink
        v-for="item in navigation"
        :key="item.to"
        :to="item.to"
        class="flex items-center gap-3 rounded-xl px-3 py-2 text-sm transition"
        :class="
          isActive(item)
            ? 'bg-brand-500/10 text-brand-200'
            : 'text-slate-400 hover:bg-white/5 hover:text-slate-100'
        "
      >
        <component :is="item.icon" class="h-4 w-4" />
        <span>{{ item.label }}</span>
      </RouterLink>
    </nav>

    <div class="border-t border-white/10 p-4">
      <div class="rounded-xl border border-white/10 bg-ink-950/60 p-3">
        <StatusPill :label="runtime.label" :tone="runtime.tone" />
        <p class="mt-3 text-[10px] font-semibold uppercase tracking-[0.18em] text-slate-600">
          Version
        </p>
        <p class="font-mono text-xs text-slate-300">v{{ appStore.displayVersion }}</p>
      </div>
    </div>
  </aside>
</template>
