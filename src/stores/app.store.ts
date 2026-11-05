import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

import { appService, isTauriRuntime, platformService } from '@/services'
import type { AppInfo, PlatformInfo } from '@/types'

export type EnvironmentStatus = 'idle' | 'loading' | 'ready' | 'browser' | 'error'

/**
 * Application shell state: runtime environment, platform capabilities and
 * error reporting. Business state must live in dedicated feature stores.
 */
export const useAppStore = defineStore('app', () => {
  const appInfo = ref<AppInfo | null>(null)
  const platformInfo = ref<PlatformInfo | null>(null)
  const environmentStatus = ref<EnvironmentStatus>('idle')
  const environmentError = ref<string | null>(null)

  const isDesktop = computed(() => isTauriRuntime())
  const isEnvironmentReady = computed(() => environmentStatus.value === 'ready')
  const platformKind = computed(() => platformInfo.value?.kind ?? 'unknown')
  const displayVersion = computed(() => appInfo.value?.version ?? '0.1.0')

  /**
   * Loads backend metadata once. Browser previews intentionally skip IPC and
   * surface a dedicated `browser` status instead of failing.
   */
  async function loadEnvironment(force = false): Promise<void> {
    if (environmentStatus.value === 'loading') {
      return
    }

    if (!force && (environmentStatus.value === 'ready' || environmentStatus.value === 'browser')) {
      return
    }

    if (!isDesktop.value) {
      environmentStatus.value = 'browser'
      return
    }

    environmentStatus.value = 'loading'
    environmentError.value = null

    try {
      const [info, platform] = await Promise.all([appService.getInfo(), platformService.getInfo()])

      appInfo.value = info
      platformInfo.value = platform
      environmentStatus.value = 'ready'
    } catch (cause) {
      environmentError.value = cause instanceof Error ? cause.message : String(cause)
      environmentStatus.value = 'error'
    }
  }

  return {
    appInfo,
    platformInfo,
    environmentStatus,
    environmentError,
    isDesktop,
    isEnvironmentReady,
    platformKind,
    displayVersion,
    loadEnvironment,
  }
})
