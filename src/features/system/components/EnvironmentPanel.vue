<script setup lang="ts">
import { AlertTriangle, Check, Loader2, Monitor, Terminal, X } from '@lucide/vue'
import { computed } from 'vue'

import SectionCard from '@/components/common/SectionCard.vue'
import { useAppStore } from '@/stores/app.store'

interface CommandContract {
  name: string
  description: string
  returns: string
}

const appStore = useAppStore()

const commands: CommandContract[] = [
  { name: 'get_app_info', description: '应用名称、版本与构建模式', returns: 'AppInfo' },
  { name: 'get_platform_info', description: '操作系统与平台适配能力', returns: 'PlatformInfo' },
  { name: 'storage_list_documents', description: '列出已持久化的文档', returns: 'string[]' },
  { name: 'storage_read_document', description: '读取指定 JSON 文档', returns: 'JsonValue | null' },
  { name: 'storage_write_document', description: '原子写入指定 JSON 文档', returns: 'void' },
  { name: 'storage_delete_document', description: '删除指定文档', returns: 'boolean' },
]

const capabilities = computed(() => {
  const capabilities = appStore.platformInfo?.capabilities
  if (!capabilities) {
    return []
  }

  return [
    { label: '系统托盘', enabled: capabilities.systemTray },
    { label: '全局快捷键', enabled: capabilities.globalShortcut },
    { label: '窗口特效', enabled: capabilities.windowEffects },
    { label: '开机自启', enabled: capabilities.autostart },
  ]
})

const environmentRows = computed(() => {
  const app = appStore.appInfo
  const platform = appStore.platformInfo
  if (!app || !platform) {
    return []
  }

  return [
    { label: '应用名称', value: app.name },
    { label: '应用版本', value: `v${app.version}` },
    { label: 'Tauri 版本', value: app.tauriVersion },
    { label: '构建模式', value: app.buildProfile },
    { label: '操作系统', value: `${platform.displayName} (${platform.os})` },
    { label: '处理器架构', value: platform.arch },
    { label: '应用数据目录', value: platform.appDataDir ?? '不可用' },
    { label: '配置目录', value: platform.configDir ?? '不可用' },
  ]
})

const windowRows = computed(() => {
  const defaults = appStore.platformInfo?.windowDefaults
  if (!defaults) {
    return []
  }

  return [
    { label: '系统标题栏', value: defaults.decorations ? '启用' : '禁用' },
    { label: '窗口透明', value: defaults.transparent ? '启用' : '禁用' },
    { label: '最小宽度', value: `${defaults.minWidth} px` },
    { label: '最小高度', value: `${defaults.minHeight} px` },
  ]
})
</script>

<template>
  <div class="space-y-4">
    <div
      v-if="appStore.environmentStatus === 'browser'"
      class="flex items-start gap-3 rounded-2xl border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-200"
    >
      <AlertTriangle class="mt-0.5 h-4 w-4 shrink-0" />
      <div>
        <p class="font-medium">当前为浏览器预览模式</p>
        <p class="mt-1 text-xs leading-5 text-amber-200/80">
          IPC 不可用，页面不会读取任何系统信息。请运行
          <code class="rounded bg-black/20 px-1 py-0.5 font-mono text-[11px]">pnpm dev:tauri</code>
          启动桌面窗口。
        </p>
      </div>
    </div>

    <div
      v-else-if="appStore.environmentStatus === 'error'"
      class="flex items-start gap-3 rounded-2xl border border-rose-500/30 bg-rose-500/10 p-4 text-sm text-rose-200"
    >
      <AlertTriangle class="mt-0.5 h-4 w-4 shrink-0" />
      <div>
        <p class="font-medium">无法读取运行环境</p>
        <p class="mt-1 break-all text-xs leading-5 text-rose-200/80">
          {{ appStore.environmentError ?? '未知错误' }}
        </p>
      </div>
    </div>

    <div v-else-if="appStore.environmentStatus === 'ready'" class="grid gap-4 lg:grid-cols-2">
      <SectionCard
        title="应用与运行时"
        description="由 Rust 命令 get_app_info 返回"
        :icon="Monitor"
      >
        <dl class="divide-y divide-white/5">
          <div
            v-for="row in environmentRows"
            :key="row.label"
            class="flex items-start justify-between gap-4 py-2.5 first:pt-0 last:pb-0"
          >
            <dt class="shrink-0 text-xs text-slate-500">{{ row.label }}</dt>
            <dd class="break-all text-right font-mono text-[11px] text-slate-300">
              {{ row.value }}
            </dd>
          </div>
        </dl>
      </SectionCard>

      <SectionCard title="平台能力" description="由 platform 适配器按操作系统声明" :icon="Terminal">
        <ul class="grid grid-cols-2 gap-2">
          <li
            v-for="capability in capabilities"
            :key="capability.label"
            class="flex items-center gap-2 rounded-lg border border-white/5 bg-white/5 px-3 py-2 text-xs"
            :class="capability.enabled ? 'text-slate-200' : 'text-slate-500'"
          >
            <Check v-if="capability.enabled" class="h-3.5 w-3.5 text-brand-400" />
            <X v-else class="h-3.5 w-3.5 text-slate-600" />
            {{ capability.label }}
          </li>
        </ul>

        <dl class="mt-4 divide-y divide-white/5">
          <div
            v-for="row in windowRows"
            :key="row.label"
            class="flex items-center justify-between gap-4 py-2"
          >
            <dt class="text-xs text-slate-500">{{ row.label }}</dt>
            <dd class="font-mono text-[11px] text-slate-300">{{ row.value }}</dd>
          </div>
        </dl>
      </SectionCard>
    </div>

    <div
      v-else
      class="flex items-center gap-3 rounded-2xl border border-white/10 bg-ink-900/70 p-6 text-sm text-slate-400"
    >
      <Loader2 class="h-4 w-4 animate-spin" />
      正在通过 IPC 读取运行环境…
    </div>

    <SectionCard
      title="IPC 命令契约"
      description="前端 services 层与 Rust commands 层的边界"
      :icon="Terminal"
    >
      <div class="overflow-x-auto">
        <table class="w-full min-w-[520px] text-left text-xs">
          <thead>
            <tr
              class="border-b border-white/10 text-[10px] uppercase tracking-wider text-slate-500"
            >
              <th class="pb-2 pr-4 font-semibold">命令</th>
              <th class="pb-2 pr-4 font-semibold">说明</th>
              <th class="pb-2 font-semibold">返回</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="command in commands" :key="command.name">
              <td class="py-2.5 pr-4 font-mono text-[11px] text-brand-200">
                {{ command.name }}
              </td>
              <td class="py-2.5 pr-4 text-slate-400">{{ command.description }}</td>
              <td class="py-2.5 font-mono text-[11px] text-slate-500">{{ command.returns }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </SectionCard>
  </div>
</template>
