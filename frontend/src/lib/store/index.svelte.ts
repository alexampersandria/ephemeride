import { useDataStore } from './dataStore.svelte'
import { registerStore } from './registerStore.svelte'
import { useUiStore } from './uiStore.svelte'
import { useUserStore } from './userStore.svelte'

export const initializeStores = () => {
  const stores = [
    { key: 'ui', store: useUiStore() },
    { key: 'user', store: useUserStore() },
    { key: 'data', store: useDataStore() },
  ]

  stores.forEach(store => {
    registerStore(store.key, store.store)
  })
}
