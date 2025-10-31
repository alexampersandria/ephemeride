import { env } from '$env/dynamic/public'
import type { CategoryWithTags, Entry } from '$lib/types/log'
import { datesInRange, monthDateRange } from '$lib/utils/log'
import { useUserStore, type UserState } from './userStore.svelte'

let userStore: UserState | null = null

/**
 * null indicates data for date has been loaded but no entry exists
 */
export type DataEntries = {
  [key: string]: Entry | null
}

export type DataState = {
  categories: CategoryWithTags[] | null
  entries: DataEntries | null
  loaded: boolean
  fetchCategories: () => Promise<void>
  fetchEntries: (from?: string, to?: string) => Promise<void>
  deleteData: () => void
}

let categories: CategoryWithTags[] | null = $state(null)
let entries: DataEntries | null = $state(null)
let loaded: boolean = $state(false)

const fetchCategories = async () => {
  if (userStore && userStore.sessionId) {
    console.log(
      'dataStore: Fetching categories with session ID:',
      userStore.sessionId,
    )

    fetch(`${env.PUBLIC_VITE_API_URL}/v1/user/categories`, {
      headers: { Authorization: `Bearer ${userStore.sessionId}` },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch categories')
        }
        return res.json()
      })
      .then((data: CategoryWithTags[]) => {
        console.log('dataStore: Fetched categories:', data)

        categories = data
      })
      .catch(err => {
        console.error('Error fetching categories:', err)
        categories = null
      })
  }
}

const fetchEntries = async (from?: string, to?: string) => {
  if (userStore && userStore.sessionId) {
    const { firstDate, lastDate } = monthDateRange()
    from = from || firstDate
    to = to || lastDate
    const dates = datesInRange(from, to)

    console.log(
      'dataStore: Fetching entries with session ID:',
      userStore.sessionId,
    )
    console.log('dataStore: From:', from, 'To:', to)

    fetch(`${env.PUBLIC_VITE_API_URL}/v1/entries/${from}/${to}`, {
      headers: { Authorization: `Bearer ${userStore.sessionId}` },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch entries')
        }
        return res.json()
      })
      .then((data: Entry[]) => {
        console.log('dataStore: Fetched entries:', data)
        const entriesByDate: DataEntries = {}
        dates.forEach(date => {
          const entry = data.find(e => e.date === date) || null
          console.log(`dataStore: Processing date ${date}:`, entry)
          entriesByDate[date] = entry
        })
        entries = entriesByDate
      })
      .catch(err => {
        console.error('Error fetching user details:', err)
        entries = null
      })
  }
}

const deleteData = () => {
  console.log('dataStore: Deleting all data')
  categories = null
  entries = null
}

export const useDataStore: () => DataState = () => {
  $effect(() => {
    if (!userStore) {
      userStore = useUserStore()
    }
  })

  $effect(() => {
    if (userStore && userStore.sessionId && !loaded) {
      console.log(
        'dataStore: User is logged in with session ID:',
        userStore.sessionId,
      )

      fetchCategories()
      fetchEntries()

      loaded = true
    }
  })

  return {
    get categories() {
      return categories
    },
    get entries() {
      return entries
    },
    set categories(value) {
      categories = value
    },
    set entries(value) {
      entries = value
    },
    get loaded() {
      return loaded
    },
    get fetchCategories() {
      return fetchCategories
    },
    get fetchEntries() {
      return fetchEntries
    },
    get deleteData() {
      return deleteData
    },
  }
}
