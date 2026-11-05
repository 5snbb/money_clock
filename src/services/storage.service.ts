import { invokeCommand } from './ipc'

import type { StorageDocument } from '@/types'

/**
 * Thin client for the Rust JSON document store.
 *
 * The backend owns validation, path resolution and atomic writes; the frontend
 * only names documents and exchanges JSON values.
 */
export const storageService = {
  list(): Promise<string[]> {
    return invokeCommand<string[]>('storage_list_documents')
  },

  read(name: string): Promise<StorageDocument | null> {
    return invokeCommand<StorageDocument | null>('storage_read_document', { name })
  },

  write(name: string, value: StorageDocument): Promise<void> {
    return invokeCommand<void>('storage_write_document', { name, value })
  },

  remove(name: string): Promise<boolean> {
    return invokeCommand<boolean>('storage_delete_document', { name })
  },
}
