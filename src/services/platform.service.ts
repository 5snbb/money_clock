import { invokeCommand } from './ipc'

import type { PlatformInfo } from '@/types'

export const platformService = {
  /** Reads OS details and adapter capabilities from the Rust platform layer. */
  getInfo(): Promise<PlatformInfo> {
    return invokeCommand<PlatformInfo>('get_platform_info')
  },
}
