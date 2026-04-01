/**
 * Erreur API conforme RFC 7807 Problem Details
 * Miroir de la chaîne DomainError → ApplicationError → ApiError du backend Rust
 */
export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly title: string,
    public readonly detail: string,
    public readonly type?: string,
  ) {
    super(title)
    this.name = 'ApiError'
  }
}

async function parseError(res: Response): Promise<ApiError> {
  try {
    const body = await res.json()
    return new ApiError(
      body.status ?? res.status,
      body.title ?? res.statusText,
      body.detail ?? '',
      body.type,
    )
  } catch {
    return new ApiError(res.status, res.statusText, '')
  }
}

const DEFAULT_TIMEOUT_MS = 10_000

function withTimeout(options: RequestInit, ms = DEFAULT_TIMEOUT_MS): RequestInit & { signal: AbortSignal } {
  const controller = new AbortController()
  setTimeout(() => controller.abort(), ms)
  return { ...options, signal: options.signal ?? controller.signal }
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiGet<T>(url: string, options?: RequestInit): Promise<T | undefined> {
  const { cache: _, method: __, ...safeOptions } = options ?? {}
  const res = await fetch(url, withTimeout({ cache: 'no-store', method: 'GET', ...safeOptions }))
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok, ou si `body` n'est pas sérialisable
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiPost<T>(url: string, body: unknown): Promise<T | undefined> {
  let serialized: string | undefined
  try {
    serialized = JSON.stringify(body)
  } catch (e) {
    throw new ApiError(-1, 'Serialization error', e instanceof Error ? e.message : String(e))
  }
  const res = await fetch(url, withTimeout({
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: serialized,
  }))
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok, ou si `body` n'est pas sérialisable
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiPatch<T>(url: string, body: unknown): Promise<T | undefined> {
  let serialized: string | undefined
  try {
    serialized = JSON.stringify(body)
  } catch (e) {
    throw new ApiError(-1, 'Serialization error', e instanceof Error ? e.message : String(e))
  }
  const res = await fetch(url, withTimeout({
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: serialized,
  }))
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiDelete(url: string): Promise<void> {
  const res = await fetch(url, withTimeout({ method: 'DELETE' }))
  if (!res.ok) throw await parseError(res)
}
