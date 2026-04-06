'use client'

import { useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import { useRegisterMaker } from '@passion-market/hooks'
import { validateRegisterMaker, RegisterMakerErrors } from '@passion-market/api-client'

type Fields = { name: string; email: string; password: string }
type Touched = { name: boolean; email: boolean; password: boolean }

export default function RegisterMakerPage() {
  const router = useRouter()
  const { status, error, execute } = useRegisterMaker()

  const [values, setValues] = useState<Fields>({ name: '', email: '', password: '' })
  const [touched, setTouched] = useState<Touched>({ name: false, email: false, password: false })
  const [fieldErrors, setFieldErrors] = useState<RegisterMakerErrors>({})

  useEffect(() => {
    if (status === 'success') {
      router.push('/auth/register/maker/confirmation')
    }
  }, [status, router])

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    const { name, value } = e.target
    const next = { ...values, [name]: value }
    setValues(next)
    if (touched[name as keyof Touched]) {
      setFieldErrors(validateRegisterMaker(next))
    }
  }

  function handleBlur(e: React.FocusEvent<HTMLInputElement>) {
    const { name } = e.target
    setTouched(t => ({ ...t, [name]: true }))
    setFieldErrors(validateRegisterMaker(values))
  }

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const allTouched: Touched = { name: true, email: true, password: true }
    setTouched(allTouched)
    const errors = validateRegisterMaker(values)
    setFieldErrors(errors)
    if (Object.keys(errors).length > 0) return
    await execute(values)
  }

  return (
    <main>
      <h1>Inscription Maker</h1>

      <form onSubmit={handleSubmit} noValidate>
        <div>
          <label htmlFor="name">Nom</label>
          <input
            id="name"
            name="name"
            type="text"
            value={values.name}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-describedby={fieldErrors.name ? 'name-error' : undefined}
          />
          {touched.name && fieldErrors.name && (
            <p id="name-error" role="alert">{fieldErrors.name}</p>
          )}
        </div>

        <div>
          <label htmlFor="email">Email</label>
          <input
            id="email"
            name="email"
            type="email"
            value={values.email}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-describedby={fieldErrors.email ? 'email-error' : undefined}
          />
          {touched.email && fieldErrors.email && (
            <p id="email-error" role="alert">{fieldErrors.email}</p>
          )}
        </div>

        <div>
          <label htmlFor="password">Mot de passe</label>
          <input
            id="password"
            name="password"
            type="password"
            value={values.password}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-describedby={fieldErrors.password ? 'password-error' : undefined}
          />
          {touched.password && fieldErrors.password && (
            <p id="password-error" role="alert">{fieldErrors.password}</p>
          )}
        </div>

        {status === 'error' && error && (
          <p role="alert">
            {error.status === 409
              ? 'Cet email est déjà utilisé.'
              : error.detail || 'Une erreur est survenue.'}
          </p>
        )}

        <button type="submit" disabled={status === 'loading'}>
          {status === 'loading' ? 'Inscription en cours…' : "S'inscrire"}
        </button>
      </form>
    </main>
  )
}
