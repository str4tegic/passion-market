/**
 * Types communs miroirs des structs Rust shared-kernel
 * Toutes les propriétés en camelCase (serde rename_all = "camelCase" côté Rust)
 */

export interface PageParams {
  page: number
  perPage: number
}

export interface Page<T> {
  items: T[]
  total: number
  page: number
  perPage: number
}
