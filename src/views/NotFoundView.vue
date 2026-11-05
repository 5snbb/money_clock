<script setup lang="ts">
import { ArrowLeft } from '@lucide/vue'
import { RouterLink } from 'vue-router'
</script>

<template>
  <div class="grid min-h-[60vh] place-items-center">
    <div class="text-center">
      <p class="font-mono text-6xl font-semibold tracking-tight text-brand-400">404</p>
      <h1 class="mt-3 text-lg font-semibold text-white">页面不存在</h1>
      <p class="mt-2 text-sm text-slate-400">该路由尚未注册到 money_clock 路由表。</p>
      <RouterLink
        to="/"
        class="mt-6 inline-flex items-center gap-2 rounded-xl bg-brand-500 px-4 py-2 text-sm font-medium text-ink-950 transition hover:bg-brand-400"
      >
        <ArrowLeft class="h-4 w-4" />
        返回概览
      </RouterLink>
    </div>
  </div>
</template>
