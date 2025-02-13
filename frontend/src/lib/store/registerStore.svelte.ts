import { browser } from '$app/environment'
import { onMount } from 'svelte'

export const registerStore = (name: string, store: any) => {
  const storeName = `store:${name}`

  const storeEffect = (store: any) => {
    const storeObject: { [key: string]: any } = {}
    const keys = Object.keys(store)
    keys.forEach(key => {
      storeObject[key] = (store as { [key: string]: any })[key]
    })
    if (browser) {
      localStorage.setItem(storeName, JSON.stringify(storeObject))
    }
  }

  const readStore = () => {
    if (browser) {
      const stored = localStorage.getItem(storeName)
      if (stored) {
        const storedObject = JSON.parse(stored)
        const keys = Object.keys(storedObject)
        keys.forEach(key => {
          if (store[key] !== storedObject[key]) {
            try {
              store[key] = storedObject[key]
            } catch (e) {}
          }
        })
      }
    }
  }

  onMount(() => {
    readStore()
  })

  $effect(() => {
    storeEffect(store)
  })
}
