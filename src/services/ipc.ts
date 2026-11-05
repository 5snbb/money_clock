import { invoke } from '@tauri-apps/api/core'

import type { IpcErrorPayload } from '@/types'

/** Normalized error raised by the IPC layer. */
export class IpcError extends Error {
  readonly code: string

  constructor(code: string, message: string) {
    super(message)
    this.name = 'IpcError'
    this.code = code
  }
}

/** Raised when the frontend runs outside of a Tauri window (e.g. browser preview). */
export class IpcUnavailableError extends IpcError {
  constructor(command: string) {
    super('ipc_unavailable', `Tauri IPC 不可用（${command}），请在桌面窗口中运行该功能。`)
    this.name = 'IpcUnavailableError'
  }
}

/** Detects whether the frontend is hosted inside a Tauri webview. */
export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

/**
 * Single entry point for calling Rust commands.
 *
 * Services must use this helper instead of importing `invoke` directly so that
 * error normalization and browser-preview guards stay in one place.
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isTauriRuntime()) {
    throw new IpcUnavailableError(command)
  }

  try {
    return await invoke<T>(command, args)
  } catch (cause) {
    throw normalizeIpcError(cause)
  }
}

function normalizeIpcError(cause: unknown): IpcError {
  if (cause instanceof IpcError) {
    return cause
  }

  if (isIpcErrorPayload(cause)) {
    return new IpcError(cause.code, cause.message)
  }

  return new IpcError('ipc_failure', cause instanceof Error ? cause.message : String(cause))
}

function isIpcErrorPayload(value: unknown): value is IpcErrorPayload {
  if (typeof value !== 'object' || value === null) {
    return false
  }

  const payload = value as Partial<IpcErrorPayload>
  return typeof payload.code === 'string' && typeof payload.message === 'string'
}
