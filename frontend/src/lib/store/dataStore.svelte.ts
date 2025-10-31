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
  getEntry: (date: string) => Promise<Entry | null>
  deleteData: () => void
}

let categories: CategoryWithTags[] | null = $state(null)
let entries: DataEntries | null = $state(null)
let loaded: boolean = $state(false)

const fetchCategories = async () => {
  if (userStore && userStore.sessionId) {
    await fetch(`${env.PUBLIC_VITE_API_URL}/v1/user/categories`, {
      headers: { Authorization: `Bearer ${userStore.sessionId}` },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch categories')
        }
        return res.json()
      })
      .then((data: CategoryWithTags[]) => {
        categories = data
      })
      .catch(err => {
        console.error('Error fetching user categories:', err)
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

    await fetch(`${env.PUBLIC_VITE_API_URL}/v1/entries/${from}/${to}`, {
      headers: { Authorization: `Bearer ${userStore.sessionId}` },
    })
      .then(res => {
        if (!res.ok) {
          throw new Error('Failed to fetch entries')
        }
        return res.json()
      })
      .then((data: Entry[]) => {
        const entriesByDate: DataEntries = {}
        dates.forEach(date => {
          let entry = data.find(e => e.date === date) || null
          // do not overwrite existing entry with null
          if (entry === null && entriesByDate[date] !== null) {
            entry = entriesByDate[date]
          }
          entriesByDate[date] = entry
        })
        entries = { ...entries, ...entriesByDate }
      })
      .catch(err => {
        console.error('Error fetching user details:', err)
        entries = null
      })
  }
}

const getEntry = async (date: string) => {
  if (entries) {
    if (date in entries) {
      return entries[date]
    } else {
      await fetchEntries(date, date)
      if (date in entries) {
        return entries[date]
      } else {
        console.error('dataStore: Entry not found for date after fetch:', date)
      }
    }
  }
  return null
}

const deleteData = () => {
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
    get getEntry() {
      return getEntry
    },
    get deleteData() {
      return deleteData
    },
  }
}
