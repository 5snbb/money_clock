<script setup lang="ts">
withDefaults(defineProps<{ compact?: boolean }>(), { compact: false })
</script>

<template>
  <div class="flex items-center gap-3">
    <svg class="h-9 w-9 shrink-0" viewBox="0 0 64 64" aria-hidden="true">
      <defs>
        <linearGradient
          id="money_clock-mark"
          x1="8"
          y1="6"
          x2="58"
          y2="60"
          gradientUnits="userSpaceOnUse"
        >
          <stop stop-color="#34d399" />
          <stop offset="1" stop-color="#0ea5e9" />
        </linearGradient>
      </defs>
      <rect width="64" height="64" rx="18" fill="#0b1220" />
      <circle
        cx="32"
        cy="32"
        r="19"
        fill="none"
        stroke="url(#money_clock-mark)"
        stroke-width="4"
        stroke-linecap="round"
        stroke-dasharray="92 28"
        transform="rotate(-90 32 32)"
      />
      <path
        d="M32 21.5V33l8 4.5"
        fill="none"
        stroke="#ecfdf5"
        stroke-width="3.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
    <div v-if="!compact" class="leading-tight">
      <p class="text-sm font-semibold tracking-wide text-white">money_clock</p>
      <p class="text-[11px] font-medium uppercase tracking-[0.18em] text-slate-500">
        Desktop Foundation
      </p>
    </div>
  </div>
</template>
