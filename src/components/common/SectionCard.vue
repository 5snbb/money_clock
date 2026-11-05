<script setup lang="ts">
import type { Component } from 'vue'

interface Props {
  title: string
  description?: string
  icon?: Component
  tone?: 'default' | 'brand'
}

withDefaults(defineProps<Props>(), { tone: 'default' })
</script>

<template>
  <section class="rounded-2xl border border-white/10 bg-ink-900/70 p-5">
    <header class="flex items-start gap-3">
      <span
        v-if="icon"
        class="grid h-9 w-9 shrink-0 place-items-center rounded-xl"
        :class="tone === 'brand' ? 'bg-brand-500/15 text-brand-300' : 'bg-white/5 text-slate-300'"
      >
        <component :is="icon" class="h-4 w-4" />
      </span>
      <div class="min-w-0">
        <h2 class="text-sm font-semibold text-white">{{ title }}</h2>
        <p v-if="description" class="mt-1 text-xs leading-5 text-slate-400">
          {{ description }}
        </p>
      </div>
      <div v-if="$slots.actions" class="ml-auto shrink-0">
        <slot name="actions" />
      </div>
    </header>
    <div class="mt-4">
      <slot />
    </div>
  </section>
</template>
