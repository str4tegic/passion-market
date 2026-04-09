import { apiPost } from '../lib/fetch'

export interface RegisterMakerInput {
  email: string
  password: string
  name: string
}

export interface RegisterMakerErrors {
  email?: string
  password?: string
  name?: string
}

export function validateRegisterMaker(input: Partial<RegisterMakerInput>): RegisterMakerErrors {
  const errors: RegisterMakerErrors = {}

  if (!input.name?.trim()) {
    errors.name = 'Le nom est requis.'
  }

  if (!input.email?.trim()) {
    errors.email = "L'email est requis."
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input.email)) {
    errors.email = "L'email n'est pas valide."
  }

  if (!input.password) {
    errors.password = 'Le mot de passe est requis.'
  } else if (input.password.length < 8) {
    errors.password = 'Le mot de passe doit contenir au moins 8 caractères.'
  }

  return errors
}

export interface RegisterMakerOutput {
  userId: string
  email: string
  role: string
}

export async function registerMaker(input: RegisterMakerInput): Promise<RegisterMakerOutput> {
  const result = await apiPost<RegisterMakerOutput>('/api/v1/auth/register/maker', input)
  return result!
}
