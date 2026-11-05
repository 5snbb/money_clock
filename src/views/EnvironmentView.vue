<script setup lang="ts">
import { onMounted } from 'vue'

import PageHeader from '@/components/common/PageHeader.vue'
import { EnvironmentPanel } from '@/features/system'
import { useAppStore } from '@/stores/app.store'

const appStore = useAppStore()

onMounted(() => {
  if (appStore.environmentStatus === 'idle') {
    void appStore.loadEnvironment()
  }
})
</script>

<template>
  <div>
    <PageHeader
      eyebrow="Runtime"
      title="运行环境"
      description="这些数据全部来自 Rust 后端：应用元信息由 services 层汇总，平台能力由 platform 适配器声明。"
    />
    <EnvironmentPanel />
  </div>
</template>
