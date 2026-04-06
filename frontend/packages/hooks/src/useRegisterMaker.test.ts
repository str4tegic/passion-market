import { describe, it, expect, vi, beforeEach } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import { useRegisterMaker } from './useRegisterMaker'
import { ApiError } from '@passion-market/api-client'

function mockFetch(status: number, body: unknown) {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    json: async () => body,
  }))
}

describe('useRegisterMaker', () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it('état initial : status idle, pas d\'erreur, pas de userId', () => {
    const { result } = renderHook(() => useRegisterMaker())

    expect(result.current.status).toBe('idle')
    expect(result.current.error).toBeUndefined()
    expect(result.current.userId).toBeUndefined()
  })

  it('status success et userId disponible après inscription réussie', async () => {
    mockFetch(201, { userId: 'u-1', email: 'f@test.com', role: 'Maker' })

    const { result } = renderHook(() => useRegisterMaker())

    await act(async () => {
      await result.current.execute({ email: 'f@test.com', password: 'S3cret!', name: 'François' })
    })

    expect(result.current.status).toBe('success')
    expect(result.current.userId).toBe('u-1')
    expect(result.current.error).toBeUndefined()
  })

  it('status error et ApiError disponible après conflit 409', async () => {
    mockFetch(409, { status: 409, title: 'Conflict', detail: 'email déjà utilisé' })

    const { result } = renderHook(() => useRegisterMaker())

    await act(async () => {
      await result.current.execute({ email: 'f@test.com', password: 'S3cret!', name: 'François' })
    })

    expect(result.current.status).toBe('error')
    expect(result.current.error).toBeInstanceOf(ApiError)
    expect(result.current.error?.status).toBe(409)
    expect(result.current.userId).toBeUndefined()
  })

  it('status error et ApiError disponible après validation 422', async () => {
    mockFetch(422, { status: 422, title: 'Unprocessable Entity', detail: 'mot de passe trop court' })

    const { result } = renderHook(() => useRegisterMaker())

    await act(async () => {
      await result.current.execute({ email: 'f@test.com', password: '1', name: 'F' })
    })

    expect(result.current.status).toBe('error')
    expect(result.current.error?.status).toBe(422)
  })

  it('repasse en idle si on rappelle reset', async () => {
    mockFetch(409, { status: 409, title: 'Conflict', detail: 'email déjà utilisé' })

    const { result } = renderHook(() => useRegisterMaker())

    await act(async () => {
      await result.current.execute({ email: 'f@test.com', password: 'S3cret!', name: 'F' })
    })

    act(() => { result.current.reset() })

    expect(result.current.status).toBe('idle')
    expect(result.current.error).toBeUndefined()
  })
})
