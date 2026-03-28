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

export async function apiGet<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(url, { cache: 'no-store', ...options })
  if (!res.ok) throw await parseError(res)
  return res.json() as Promise<T>
}

export async function apiPost<T>(url: string, body: unknown): Promise<T> {
  const res = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw await parseError(res)
  return res.json() as Promise<T>
}

export async function apiPatch<T>(url: string, body: unknown): Promise<T> {
  const res = await fetch(url, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw await parseError(res)
  return res.json() as Promise<T>
}

export async function apiDelete(url: string): Promise<void> {
  const res = await fetch(url, { method: 'DELETE' })
  if (!res.ok) throw await parseError(res)
}
