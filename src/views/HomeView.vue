<script setup lang="ts">
import { ArrowRight, Boxes, Cpu, Layers, ShieldCheck, Terminal } from '@lucide/vue'
import { computed } from 'vue'
import { RouterLink } from 'vue-router'

import PageHeader from '@/components/common/PageHeader.vue'
import SectionCard from '@/components/common/SectionCard.vue'
import StatusPill from '@/components/common/StatusPill.vue'
import { useAppStore } from '@/stores/app.store'

const appStore = useAppStore()

const runtime = computed(() => {
  switch (appStore.environmentStatus) {
    case 'ready':
      return { label: 'Rust 后端已连接', tone: 'success' as const }
    case 'browser':
      return { label: '浏览器预览模式', tone: 'warning' as const }
    case 'error':
      return { label: '后端连接失败', tone: 'danger' as const }
    case 'loading':
      return { label: '正在连接后端', tone: 'info' as const }
    default:
      return { label: '尚未检测', tone: 'neutral' as const }
  }
})

const frontendLayers = [
  { title: 'views · layouts', description: '页面与布局只做编排和组合，不直接访问系统能力。' },
  { title: 'components', description: '跨模块复用的展示型组件，保持无业务状态。' },
  { title: 'features', description: '按业务域组织模块，后续功能在各自目录内扩展。' },
  { title: 'services · stores', description: 'services 封装 IPC，stores 管理跨组件状态。' },
]

const rustModules = [
  { title: 'commands', description: 'IPC 入口，负责参数校验与调用编排。' },
  { title: 'services', description: '应用与平台信息的服务层编排。' },
  { title: 'storage', description: 'JSON 文档原子读写，路径由后端解析。' },
  { title: 'platform', description: 'Windows / macOS / Linux 平台适配器。' },
]

const platforms = ['Windows', 'macOS', 'Linux']
</script>

<template>
  <div>
    <PageHeader
      eyebrow="money_clock · Foundation"
      title="桌面应用基础架构已就绪"
      description="Vue 3 + TypeScript + Vite 负责界面与交互，Tauri 2 + Rust 负责系统能力，两侧通过明确的 IPC 契约协作。"
    />

    <div class="grid gap-4 lg:grid-cols-3">
      <SectionCard
        title="前端分层"
        description="按职责拆分，业务代码不触碰系统 API"
        :icon="Layers"
        tone="brand"
      >
        <ul class="space-y-2.5">
          <li
            v-for="layer in frontendLayers"
            :key="layer.title"
            class="flex gap-2.5 text-xs leading-5"
          >
            <span class="mt-2 h-1 w-1 shrink-0 rounded-full bg-brand-400/80" />
            <span>
              <code class="font-mono text-[11px] text-brand-200">{{ layer.title }}</code>
              <span class="text-slate-400"> — {{ layer.description }}</span>
            </span>
          </li>
        </ul>
      </SectionCard>

      <SectionCard
        title="Rust 后端"
        description="commands / services / storage / platform 四层解耦"
        :icon="Cpu"
      >
        <ul class="space-y-2.5">
          <li
            v-for="module in rustModules"
            :key="module.title"
            class="flex gap-2.5 text-xs leading-5"
          >
            <span class="mt-2 h-1 w-1 shrink-0 rounded-full bg-slate-500" />
            <span>
              <code class="font-mono text-[11px] text-slate-200">{{ module.title }}</code>
              <span class="text-slate-400"> — {{ module.description }}</span>
            </span>
          </li>
        </ul>
      </SectionCard>

      <SectionCard title="平台适配" description="能力差异收敛在适配器内部" :icon="Boxes">
        <div class="flex flex-wrap gap-2">
          <span
            v-for="platform in platforms"
            :key="platform"
            class="rounded-lg border border-white/10 bg-white/5 px-2.5 py-1 font-mono text-[11px] text-slate-300"
          >
            {{ platform }}
          </span>
        </div>
        <p class="mt-3 text-xs leading-5 text-slate-400">
          平台模块声明系统托盘、全局快捷键、窗口特效、开机自启等能力，不包含业务判断。
        </p>
      </SectionCard>
    </div>

    <div class="mt-4 grid gap-4 lg:grid-cols-5">
      <SectionCard
        class="lg:col-span-3"
        title="启动检查"
        description="运行状态由前端 services 层通过 IPC 获取"
        :icon="Terminal"
      >
        <div class="flex flex-wrap items-center gap-3">
          <StatusPill :label="runtime.label" :tone="runtime.tone" pulse />
          <span class="text-xs text-slate-500">
            {{ appStore.isDesktop ? '桌面窗口' : '浏览器预览（IPC 不可用）' }}
          </span>
        </div>
        <p class="mt-3 text-xs leading-5 text-slate-400">
          应用启动时会并行读取应用元信息与平台信息。若在浏览器中预览，界面会进入降级模式，而不是抛出错误。
        </p>
        <RouterLink
          to="/environment"
          class="mt-4 inline-flex items-center gap-1.5 text-xs font-medium text-brand-300 transition hover:text-brand-200"
        >
          查看运行环境
          <ArrowRight class="h-3.5 w-3.5" />
        </RouterLink>
      </SectionCard>

      <SectionCard class="lg:col-span-2" title="架构原则" :icon="ShieldCheck">
        <ul class="space-y-2 text-xs leading-5 text-slate-400">
          <li>系统能力只存在于 Rust 层。</li>
          <li>业务逻辑不直接依赖 Tauri API。</li>
          <li>前后端契约集中在 types / services。</li>
          <li>业务功能尚未实现，待 features 扩展。</li>
        </ul>
      </SectionCard>
    </div>
  </div>
</template>
