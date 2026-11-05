<script setup lang="ts">
interface Props {
  title: string
  description?: string
  eyebrow?: string
}

defineProps<Props>()
</script>

<template>
  <header class="mb-6 flex flex-col gap-1">
    <p v-if="eyebrow" class="text-[11px] font-semibold uppercase tracking-[0.22em] text-brand-400">
      {{ eyebrow }}
    </p>
    <h1 class="text-2xl font-semibold tracking-tight text-white">{{ title }}</h1>
    <p v-if="description" class="max-w-3xl text-sm leading-6 text-slate-400">
      {{ description }}
    </p>
  </header>
</template>
