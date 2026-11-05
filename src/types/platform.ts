export type PlatformKind = 'windows' | 'macos' | 'linux' | 'unknown'

/** Capabilities declared by the Rust platform adapter for the current OS. */
export interface PlatformCapabilities {
  systemTray: boolean
  globalShortcut: boolean
  windowEffects: boolean
  autostart: boolean
}

/** Window defaults recommended by the platform adapter. */
export interface WindowDefaults {
  decorations: boolean
  transparent: boolean
  minWidth: number
  minHeight: number
}

export interface PlatformInfo {
  kind: PlatformKind
  displayName: string
  os: string
  arch: string
  family: string
  appDataDir: string | null
  configDir: string | null
  capabilities: PlatformCapabilities
  windowDefaults: WindowDefaults
}
