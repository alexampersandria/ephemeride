import { registerStore } from './registerStore.svelte'
import { useUiStore } from './uiStore.svelte'

export const initializeStores = () => {
  const stores = [{ key: 'ui', store: useUiStore() }]

  stores.forEach(store => {
    registerStore(store.key, store.store)
  })
}
