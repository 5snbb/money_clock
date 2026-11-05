export type JsonValue =
  string | number | boolean | null | JsonValue[] | { [key: string]: JsonValue }

/** A JSON document persisted by the Rust storage module. */
export type StorageDocument = JsonValue
