import { env } from '$env/dynamic/public'
import type {
  Category,
  CategoryWithTags,
  EditCategory,
  EditEntry,
  EditTag,
  Entry,
  NewCategory,
  NewEntry,
  NewTag,
  Tag,
} from '$lib/types/log'
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

  getAllEntries: () => Promise<Entry[] | null>

  getEntry: (date: string) => Promise<Entry | null>

  createEntry: (entry: NewEntry) => Promise<Entry | null>
  updateEntry: (entry: EditEntry) => Promise<Entry | null>
  deleteEntry: (id: string) => Promise<void>

  createCategory: (category: NewCategory) => Promise<CategoryWithTags | null>
  updateCategory: (category: EditCategory) => Promise<CategoryWithTags | null>
  deleteCategory: (id: string) => Promise<void>

  createTag: (tag: NewTag) => Promise<Tag | null>
  updateTag: (tag: EditTag) => Promise<Tag | null>
  deleteTag: (id: string) => Promise<void>

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

const getAllEntries = async (): Promise<Entry[] | null> => {
  const res = await fetch(
    `${env.PUBLIC_VITE_API_URL}/v1/entries/0001-01-01/9999-12-31`,
    {
      headers: { Authorization: `Bearer ${userStore?.sessionId}` },
    },
  )
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to fetch all entries')
      }
      return res.json() as Promise<Entry[]>
    })
    .catch(err => {
      console.error('Error fetching all entries:', err)
      entries = null
    })

  return res || null
}

const createEntry = async (entry: NewEntry): Promise<Entry | null> => {
  const e = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(entry),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create entry')
      }
      return res.json() as Promise<Entry>
    })
    .catch(err => {
      console.error('Error creating entry:', err)
      return null
    })

  if (e) {
    if (entries) {
      entries[e.date] = e
    } else {
      entries = { [e.date]: e }
    }

    return e
  }
  return null
}

const updateEntry = async (entry: EditEntry): Promise<Entry | null> => {
  const e = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry/${entry.id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(entry),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update entry')
      }
      return res.json() as Promise<Entry>
    })
    .catch(err => {
      console.error('Error updating entry:', err)
      return null
    })

  if (e) {
    if (entries) {
      entries[e.date] = e
    } else {
      entries = { [e.date]: e }
    }

    return e
  }
  return null
}

const deleteEntry = async (id: string): Promise<void> => {
  const deleted = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete entry')
      }

      if (res.status === 204) {
        return true
      }
    })
    .catch(err => {
      console.error('Error deleting entry:', err)
    })

  if (deleted && entries) {
    const entryToDelete = Object.values(entries).find(e => e?.id === id)
    if (entryToDelete) {
      delete entries[entryToDelete.date]
    }
  }
}

const createCategory = async (
  category: NewCategory,
): Promise<CategoryWithTags | null> => {
  const cat = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/category`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(category),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create category')
      }
      return res.json() as Promise<Category>
    })
    .catch(err => {
      console.error('Error creating category:', err)
      return null
    })

  if (cat) {
    if (categories) {
      categories.push({ ...cat, tags: [] })
    }

    return { ...cat, tags: [] }
  }
  return null
}

const updateCategory = async (
  category: EditCategory,
): Promise<CategoryWithTags | null> => {
  const cat = await fetch(
    `${env.PUBLIC_VITE_API_URL}/v1/category/${category.id}`,
    {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${userStore?.sessionId}`,
      },
      body: JSON.stringify({ name: category.name }),
    },
  )
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update category')
      }
      return res.json() as Promise<Category>
    })
    .catch(err => {
      console.error('Error updating category:', err)
      return null
    })

  if (cat) {
    if (categories) {
      categories = categories.map(c => {
        if (c.id === cat.id) {
          return { ...c, name: cat.name }
        }
        return c
      })

      return categories.find(c => c.id === cat.id) || null
    }
  }
  return null
}

const deleteCategory = async (id: string): Promise<void> => {
  const deleted = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/category/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete category')
      }

      if (res.status === 204) {
        return true
      }
    })
    .catch(err => {
      console.error('Error deleting category:', err)
    })

  if (deleted && categories) {
    categories = categories.filter(c => c.id !== id)
  }
}

const createTag = async (tag: NewTag): Promise<Tag | null> => {
  const t = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify(tag),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to create tag')
      }
      return res.json() as Promise<Tag>
    })
    .catch(err => {
      console.error('Error creating tag:', err)
      return null
    })

  if (t) {
    const tagCategory = categories?.find(c => c.id === t.category_id)
    if (tagCategory) {
      tagCategory.tags.push(t)
    }

    return t
  }
  return null
}

const updateTag = async (tag: EditTag): Promise<Tag | null> => {
  const t = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag/${tag.id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify({
      name: tag.name,
      color: tag.color,
    }),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update tag')
      }
      return res.json() as Promise<Tag>
    })
    .catch(err => {
      console.error('Error updating tag:', err)
      return null
    })

  if (t) {
    if (categories) {
      const tagCategory = categories.find(c => c.id === t.category_id)
      if (tagCategory) {
        tagCategory.tags = tagCategory.tags.map(ct => (ct.id === t.id ? t : ct))
      }
    }

    return t
  }
  return null
}

const deleteTag = async (id: string): Promise<void> => {
  const deleted = await fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to delete tag')
      }

      if (res.status === 204) {
        return true
      }
    })
    .catch(err => {
      console.error('Error deleting tag:', err)
    })

  if (deleted && categories) {
    categories.forEach(c => {
      c.tags = c.tags.filter(t => t.id !== id)
    })
  }
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
    get getAllEntries() {
      return getAllEntries
    },

    get createEntry() {
      return createEntry
    },
    get updateEntry() {
      return updateEntry
    },
    get deleteEntry() {
      return deleteEntry
    },

    get createCategory() {
      return createCategory
    },
    get updateCategory() {
      return updateCategory
    },
    get deleteCategory() {
      return deleteCategory
    },

    get createTag() {
      return createTag
    },
    get updateTag() {
      return updateTag
    },
    get deleteTag() {
      return deleteTag
    },

    get deleteData() {
      return deleteData
    },
  }
}
