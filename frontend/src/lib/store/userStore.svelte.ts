import { env } from '$env/dynamic/public'
import type { UserDetails } from '$lib/types/user'
import { goto } from '$app/navigation'
import { useDataStore, type DataState } from './dataStore.svelte'

let dataStore: DataState | null = null

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
  if (dataStore) {
    dataStore.deleteData()
  }
  goto('/')
}

const logIn = (newSessionId: string) => {
  sessionId = newSessionId
  if (dataStore) {
    dataStore.fetchCategories()
  }
  goto('/app')
}

export const useUserStore: () => UserState = () => {
  $effect(() => {
    if (!dataStore) {
      dataStore = useDataStore()
    }
  })

  $effect(() => {
    if (sessionId) {
      fetch(`${env.PUBLIC_VITE_API_URL}/v1/user`, {
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
    set userDetails(value) {
      userDetails = value
    },
    get logOut() {
      return logOut
    },
    get logIn() {
      return logIn
    },
  }
}
