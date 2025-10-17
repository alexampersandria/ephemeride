import type { UserDetails } from '$lib/types/user'
import { env } from '$env/dynamic/public'
import { goto } from '$app/navigation'

export const themes = ['dark', 'light', 'system'] as const
export type Theme = (typeof themes)[number]

export type UserState = {
  sessionId: string | null
  userDetails: UserDetails | null
  logOut: () => void
  logIn: (sessionId: string) => void
}

let sessionId: string | null = $state(null)
let userDetails: UserDetails | null = $state(null)

const logOut = () => {
  sessionId = null
  userDetails = null
  goto('/')
}

const logIn = (newSessionId: string) => {
  console.log('Logging in with session ID:', newSessionId)
  sessionId = newSessionId
  console.log('sessionId set to:', sessionId)
  goto('/app')
}

export const useUserStore: () => UserState = () => {
  $effect(() => {
    if (sessionId) {
      fetch(env.PUBLIC_VITE_API_URL + '/v1/user', {
        headers: { Authorization: `Bearer ${sessionId}` },
      })
        .then(res => {
          if (!res.ok) {
            throw new Error('Failed to fetch user details')
          }
          return res.json()
        })
        .then((data: UserDetails) => {
          userDetails = data
        })
        .catch(err => {
          console.error('Error fetching user details:', err)
          userDetails = null
        })
    }
  })

  return {
    get sessionId() {
      return sessionId
    },
    set sessionId(value) {
      sessionId = value
    },
    get userDetails() {
      return userDetails
    },
    get logOut() {
      return logOut
    },
    get logIn() {
      return logIn
    },
  }
}
