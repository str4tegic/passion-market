import { useState } from 'react'
import { registerMaker, RegisterMakerInput } from '@passion-market/api-client'
import { ApiError } from '@passion-market/api-client'

type State =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; userId: string }
  | { status: 'error'; error: ApiError }

export function useRegisterMaker() {
  const [state, setState] = useState<State>({ status: 'idle' })

  const execute = async (input: RegisterMakerInput): Promise<void> => {
    setState({ status: 'loading' })
    try {
      const result = await registerMaker(input)
      setState({ status: 'success', userId: result.userId })
    } catch (e) {
      const error = e instanceof ApiError
        ? e
        : new ApiError(-1, 'Unknown error', String(e))
      setState({ status: 'error', error })
    }
  }

  const reset = () => setState({ status: 'idle' })

  return {
    status: state.status,
    userId: state.status === 'success' ? state.userId : undefined,
    error: state.status === 'error' ? state.error : undefined,
    execute,
    reset,
  }
}
