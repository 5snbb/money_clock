/** Shape of the error object serialized by the Rust `AppError` type. */
export interface IpcErrorPayload {
  code: string
  message: string
}
