<script setup lang="ts">
import { RouterView } from 'vue-router'

import AppSidebar from '@/components/layout/AppSidebar.vue'
import AppTopbar from '@/components/layout/AppTopbar.vue'
</script>

<template>
  <div class="flex h-full min-h-screen bg-ink-950">
    <AppSidebar class="hidden md:flex" />

    <div class="flex min-w-0 flex-1 flex-col">
      <AppTopbar />

      <main class="flex-1 overflow-y-auto px-6 py-6">
        <div class="mx-auto w-full max-w-5xl">
          <RouterView v-slot="{ Component }">
            <Transition name="fade" mode="out-in">
              <component :is="Component" />
            </Transition>
          </RouterView>
        </div>
      </main>
    </div>
  </div>
</template>
