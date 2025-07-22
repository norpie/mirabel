// Manual TypeScript definition for ApiResponse<T>
// This is a simple, stable interface that doesn't need ts-rs generation

export interface ApiResponse<T> {
  data: T | null;
  error: string | null;
}