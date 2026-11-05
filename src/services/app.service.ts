import { invokeCommand } from './ipc'

import type { AppInfo } from '@/types'

export const appService = {
  /** Reads application metadata from the Rust `commands::app` module. */
  getInfo(): Promise<AppInfo> {
    return invokeCommand<AppInfo>('get_app_info')
  },
}
