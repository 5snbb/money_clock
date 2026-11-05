import { createApp } from 'vue'

import App from './App.vue'
import './assets/styles/main.css'
import { router } from './router'
import { pinia } from './stores'
import { useAppStore } from './stores/app.store'

async function bootstrap(): Promise<void> {
  const app = createApp(App)

  app.use(pinia)
  app.use(router)

  // Resolve the runtime environment before mounting so the shell renders a
  // stable state (ready, browser preview or error) on first paint.
  await useAppStore(pinia).loadEnvironment()
  await router.isReady()

  app.mount('#app')
}

bootstrap().catch((error: unknown) => {
  console.error('[money_clock] 应用启动失败', error)
})
