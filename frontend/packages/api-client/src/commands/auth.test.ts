import { describe, it, expect, vi, beforeEach } from 'vitest'
import { registerMaker, validateRegisterMaker } from './auth'
import { ApiError } from '../lib/fetch'

describe('registerMaker', () => {
  beforeEach(() => {
    vi.restoreAllMocks()
  })

  it('envoie POST /api/v1/auth/register/maker avec email, password, name', async () => {
    const mockFetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 201,
      json: async () => ({ userId: 'u-1', email: 'f@test.com', role: 'Maker' }),
    })
    vi.stubGlobal('fetch', mockFetch)

    await registerMaker({ email: 'f@test.com', password: 'S3cret!', name: 'François' })

    expect(mockFetch).toHaveBeenCalledWith(
      '/api/v1/auth/register/maker',
      expect.objectContaining({
        method: 'POST',
        headers: expect.objectContaining({ 'Content-Type': 'application/json' }),
        body: JSON.stringify({ email: 'f@test.com', password: 'S3cret!', name: 'François' }),
      }),
    )
  })

  it('retourne { userId, email, role } en cas de succès', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      status: 201,
      json: async () => ({ userId: 'u-1', email: 'f@test.com', role: 'Maker' }),
    }))

    const result = await registerMaker({ email: 'f@test.com', password: 'S3cret!', name: 'François' })

    expect(result).toEqual({ userId: 'u-1', email: 'f@test.com', role: 'Maker' })
  })

  it('lance ApiError 409 si email déjà utilisé', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 409,
      json: async () => ({ status: 409, title: 'Conflict', detail: 'email déjà utilisé' }),
    }))

    await expect(
      registerMaker({ email: 'f@test.com', password: 'S3cret!', name: 'François' }),
    ).rejects.toThrow(ApiError)
  })

  it('lance ApiError 422 si la validation échoue', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: false,
      status: 422,
      json: async () => ({ status: 422, title: 'Unprocessable Entity', detail: 'mot de passe trop court' }),
    }))

    const err = await registerMaker({ email: 'f@test.com', password: '1', name: 'F' }).catch(e => e)

    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(422)
    expect(err.detail).toBe('mot de passe trop court')
  })
})

describe('validateRegisterMaker', () => {
  it('pas d\'erreur si tous les champs sont valides', () => {
    expect(validateRegisterMaker({ email: 'f@test.com', password: 'S3cret!1', name: 'François' })).toEqual({})
  })

  it('erreur name si vide', () => {
    expect(validateRegisterMaker({ email: 'f@test.com', password: 'S3cret!1', name: '' })).toHaveProperty('name')
  })

  it('erreur email si vide', () => {
    expect(validateRegisterMaker({ email: '', password: 'S3cret!1', name: 'François' })).toHaveProperty('email')
  })

  it('erreur email si format invalide (sans @)', () => {
    expect(validateRegisterMaker({ email: 'pas-un-email', password: 'S3cret!1', name: 'François' })).toHaveProperty('email')
  })

  it('erreur email si format invalide (sans domaine)', () => {
    expect(validateRegisterMaker({ email: 'f@', password: 'S3cret!1', name: 'François' })).toHaveProperty('email')
  })

  it('erreur password si moins de 8 caractères', () => {
    expect(validateRegisterMaker({ email: 'f@test.com', password: '1234567', name: 'François' })).toHaveProperty('password')
  })

  it('pas d\'erreur password si exactement 8 caractères', () => {
    const errors = validateRegisterMaker({ email: 'f@test.com', password: '12345678', name: 'François' })
    expect(errors.password).toBeUndefined()
  })
})
