import { env } from '$env/dynamic/public'
import type { Entry } from '$lib/types/log'
import type { Session } from '$lib/types/user'

export type FetchEntriesOptions = {
  from_date?: string
  to_date?: string
  tags?: string[]
  from_mood?: number
  to_mood?: number
  order?: 'date_asc' | 'date_desc'
}

export const getEntries = async (
  sessionId: string,
  options?: FetchEntriesOptions,
): Promise<Entry[] | void> => {
  const params = new URLSearchParams()
  if (options?.from_date) {
    params.append('from_date', options.from_date)
  }
  if (options?.to_date) {
    params.append('to_date', options.to_date)
  }
  if (options?.tags && options.tags.length > 0) {
    params.append('tags', options.tags.join(','))
  }
  if (options?.from_mood) {
    params.append('from_mood', `${options.from_mood}`)
  }
  if (options?.to_mood) {
    params.append('to_mood', `${options.to_mood}`)
  }
  if (options?.order) {
    params.append('order', options.order)
  }
  const url = new URL(`${env.PUBLIC_VITE_API_URL}/v1/entries`)
  url.search = params.toString()

  return fetch(url, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch entries')
      }
      return res.json()
    })
    .then((data: Entry[]) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching user details:', err)
    })
}

export const getSessions = (sessionId: string) => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/sessions`, {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch sessions')
      }
      return res.json()
    })
    .then((data: Session[]) => {
      return data
    })
    .catch(err => {
      console.error('Error fetching sessions:', err)
    })
}
