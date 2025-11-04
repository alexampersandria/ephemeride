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
  TagWithCategory,
} from '$lib/types/log'
import { getEntries, type FetchEntriesOptions } from '$lib/utils/api'
import { monthDateRange, sortCategories, sortEntries } from '$lib/utils/log'
import { useUserStore, type UserState } from './userStore.svelte'

let userStore: UserState | null = null

export type DataState = {
  categories: CategoryWithTags[]
  entries: Entry[]
  loaded: boolean
  fetchCategories: () => Promise<void>
  fetchEntries: (options?: FetchEntriesOptions) => Promise<void>

  fetchEntry: (date: string) => Promise<Entry | null>

  getTag: (id: string) => TagWithCategory | null
  getTags: () => TagWithCategory[] | null
  getEntry: (date: string) => Entry | null

  createEntry: (entry: NewEntry) => Promise<Entry | null>
  updateEntry: (entry: EditEntry) => Promise<Entry | null>
  deleteEntry: (id: string) => Promise<boolean | null>

  createCategory: (category: NewCategory) => Promise<CategoryWithTags | null>
  updateCategory: (category: EditCategory) => Promise<CategoryWithTags | null>
  deleteCategory: (id: string) => Promise<boolean | null>

  createTag: (tag: NewTag) => Promise<Tag | null>
  updateTag: (tag: EditTag) => Promise<Tag | null>
  deleteTag: (id: string) => Promise<boolean | null>

  deleteData: () => void
}

let categories: CategoryWithTags[] = $state([])
let entries: Entry[] = $state([])
let loaded: boolean = $state(false)

const fetchCategories = async () => {
  await fetch(`${env.PUBLIC_VITE_API_URL}/v1/user/categories`, {
    headers: { Authorization: `Bearer ${userStore?.sessionId}` },
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
    })
}

const fetchEntries = async (options?: FetchEntriesOptions) => {
  const { firstDate, lastDate } = monthDateRange()
  if (userStore?.sessionId) {
    const data = await getEntries(
      userStore.sessionId,
      // if no options provided, fetch current month
      options || {
        from_date: firstDate,
        to_date: lastDate,
      },
    )
    if (data) {
      data.forEach(entry => {
        if (getEntry(entry.date)) {
          // update existing entry
          entries = entries.map(e => (e.date === entry.date ? entry : e))
        } else {
          // add new entry
          entries.push(entry)
        }
      })
    }
  }
}

const fetchEntry = async (date: string) => {
  if (entries) {
    const entry = getEntry(date)
    if (entry) {
      return entry
    } else {
      await fetchEntries({
        from_date: date,
        to_date: date,
      })
      return getEntry(date)
    }
  }
  return null
}

const getTag = (id: string): TagWithCategory | null => {
  if (categories) {
    for (const category of categories) {
      const tag = category.tags.find(t => t.id === id)
      if (tag) {
        return { ...tag, category }
      }
    }
  }
  return null
}

const getTags = (): TagWithCategory[] | null => {
  if (categories) {
    const sorted = sortCategories(categories)
    let tags: TagWithCategory[] = []
    sorted.forEach(category => {
      tags = tags.concat(category.tags.map(tag => ({ ...tag, category })))
    })
    return tags
  }
  return null
}

const getEntry = (date: string): Entry | null => {
  if (entries) {
    const entry = entries.find(e => e.date === date)
    if (entry) {
      return entry
    }
  }
  return null
}

const createEntry = async (entry: NewEntry): Promise<Entry | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry`, {
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
      return res.json()
    })
    .then((data: Entry) => {
      if (entries) {
        entries.push(data)
      } else {
        entries = [data]
      }

      return data
    })
    .catch(err => {
      console.error('Error creating entry:', err)
      return null
    })
}

const updateEntry = async (entry: EditEntry): Promise<Entry | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry/${entry.id}`, {
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
      return res.json()
    })
    .then((data: Entry) => {
      if (entries) {
        entries = entries.map(e => (e.date === data.date ? data : e))
      } else {
        entries = [data]
      }

      return data
    })
    .catch(err => {
      console.error('Error updating entry:', err)
      return null
    })
}

const deleteEntry = async (id: string): Promise<boolean | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/entry/${id}`, {
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
      } else {
        return false
      }
    })
    .then(data => {
      if (data && entries) {
        entries = entries.filter(e => e.id !== id)
      }

      return data
    })
    .catch(err => {
      console.error('Error deleting entry:', err)
      return null
    })
}

const createCategory = async (
  category: NewCategory,
): Promise<CategoryWithTags | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/category`, {
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
      return res.json()
    })
    .then((data: Category) => {
      if (categories) {
        categories.push({ ...data, tags: [] })
      }
      return { ...data, tags: [] }
    })
    .catch(err => {
      console.error('Error creating category:', err)
      return null
    })
}

const updateCategory = async (
  category: EditCategory,
): Promise<CategoryWithTags | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/category/${category.id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${userStore?.sessionId}`,
    },
    body: JSON.stringify({ name: category.name }),
  })
    .then(res => {
      if (!res.ok) {
        throw new Error('Failed to update category')
      }
      return res.json()
    })
    .then((data: Category) => {
      if (categories) {
        categories = categories.map(c => {
          if (c.id === data.id) {
            return { ...c, name: data.name }
          }
          return c
        })

        return categories.find(c => c.id === data.id) || null
      } else {
        return null
      }
    })
    .catch(err => {
      console.error('Error updating category:', err)
      return null
    })
}

const deleteCategory = async (id: string): Promise<boolean | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/category/${id}`, {
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
      } else {
        return false
      }
    })
    .then(data => {
      if (data && categories) {
        categories = categories.filter(c => c.id !== id)
      }
      return data
    })
    .catch(err => {
      console.error('Error deleting category:', err)
      return null
    })
}

const createTag = async (tag: NewTag): Promise<Tag | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag`, {
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
      return res.json()
    })
    .then((data: Tag) => {
      if (categories) {
        const tagCategory = categories.find(c => c.id === data.category_id)
        if (tagCategory) {
          tagCategory.tags.push(data)
        }
      }
      return data
    })
    .catch(err => {
      console.error('Error creating tag:', err)
      return null
    })
}

const updateTag = async (tag: EditTag): Promise<Tag | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag/${tag.id}`, {
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
      return res.json()
    })
    .then((data: Tag) => {
      if (categories) {
        const tagCategory = categories.find(c => c.id === data.category_id)
        if (tagCategory) {
          tagCategory.tags = tagCategory.tags.map(t =>
            t.id === data.id ? data : t,
          )
        }
      }

      return data
    })
    .catch(err => {
      console.error('Error updating tag:', err)
      return null
    })
}

const deleteTag = async (id: string): Promise<boolean | null> => {
  return fetch(`${env.PUBLIC_VITE_API_URL}/v1/tag/${id}`, {
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
      } else {
        return false
      }
    })
    .then(data => {
      if (categories) {
        categories.forEach(c => {
          c.tags = c.tags.filter(t => t.id !== id)
        })
      }
      return data
    })
    .catch(err => {
      console.error('Error deleting tag:', err)
      return null
    })
}

const deleteData = () => {
  categories = []
  entries = []
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
      if (categories) {
        return sortCategories(categories)
      }
      return []
    },
    get entries() {
      if (entries) {
        return sortEntries(entries)
      }
      return []
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

    get fetchEntry() {
      return fetchEntry
    },

    get getTag() {
      return getTag
    },
    get getTags() {
      return getTags
    },
    get getEntry() {
      return getEntry
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
