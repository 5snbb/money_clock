<script setup lang="ts">
import { computed } from 'vue'

type Tone = 'neutral' | 'success' | 'warning' | 'danger' | 'info'

const props = withDefaults(
  defineProps<{
    label: string
    tone?: Tone
    pulse?: boolean
  }>(),
  {
    tone: 'neutral',
    pulse: false,
  },
)

const toneClasses = computed(() => {
  switch (props.tone) {
    case 'success':
      return 'border-brand-500/30 bg-brand-500/10 text-brand-300'
    case 'warning':
      return 'border-amber-500/30 bg-amber-500/10 text-amber-300'
    case 'danger':
      return 'border-rose-500/30 bg-rose-500/10 text-rose-300'
    case 'info':
      return 'border-sky-500/30 bg-sky-500/10 text-sky-300'
    default:
      return 'border-white/10 bg-white/5 text-slate-300'
  }
})

const dotClasses = computed(() => {
  switch (props.tone) {
    case 'success':
      return 'bg-brand-400'
    case 'warning':
      return 'bg-amber-400'
    case 'danger':
      return 'bg-rose-400'
    case 'info':
      return 'bg-sky-400'
    default:
      return 'bg-slate-400'
  }
})
</script>

<template>
  <span
    class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium"
    :class="toneClasses"
  >
    <span class="relative flex h-1.5 w-1.5">
      <span
        v-if="pulse"
        class="absolute inline-flex h-full w-full animate-ping rounded-full opacity-75"
        :class="dotClasses"
      />
      <span class="relative inline-flex h-1.5 w-1.5 rounded-full" :class="dotClasses" />
    </span>
    {{ label }}
  </span>
</template>
