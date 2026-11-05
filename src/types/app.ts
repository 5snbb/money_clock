export interface AppInfo {
  /** Application display name resolved from the Tauri configuration. */
  name: string
  /** Application version from Cargo.toml. */
  version: string
  /** Version of the Tauri runtime compiled into the backend. */
  tauriVersion: string
  /** Operating system reported by the Rust standard library. */
  os: string
  /** CPU architecture reported by the Rust standard library. */
  arch: string
  /** Cargo build profile: `debug` or `release`. */
  buildProfile: string
  /** Whether the backend was compiled with debug assertions. */
  debug: boolean
}
