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

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiGet<T>(url: string, options?: RequestInit): Promise<T> {
  const { cache: _, method: __, ...safeOptions } = options ?? {}
  const res = await fetch(url, { cache: 'no-store', method: 'GET', ...safeOptions })
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined as T
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok, ou si `body` n'est pas sérialisable
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiPost<T>(url: string, body: unknown): Promise<T> {
  let serialized: string
  try {
    serialized = JSON.stringify(body)
  } catch (e) {
    throw new ApiError(0, 'Serialization error', String(e))
  }
  const res = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: serialized,
  })
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined as T
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok, ou si `body` n'est pas sérialisable
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiPatch<T>(url: string, body: unknown): Promise<T> {
  let serialized: string
  try {
    serialized = JSON.stringify(body)
  } catch (e) {
    throw new ApiError(0, 'Serialization error', String(e))
  }
  const res = await fetch(url, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: serialized,
  })
  if (!res.ok) throw await parseError(res)
  if (res.status === 204) return undefined as T
  return res.json() as Promise<T>
}

/**
 * @throws {ApiError} si le serveur répond avec un status non-ok
 * @throws {TypeError} si la requête échoue au niveau réseau (ECONNREFUSED, DNS, etc.)
 */
export async function apiDelete(url: string): Promise<void> {
  const res = await fetch(url, { method: 'DELETE' })
  if (!res.ok) throw await parseError(res)
}
