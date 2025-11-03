import { browser } from '$app/environment'
import { page } from '$app/state'

export const getRoutes = (filter?: RegExp) => {
  const routes = Object.keys(
    import.meta.glob('/src/routes/**/+page.*', { eager: true }),
  )
  const filteredRoutes = filter
    ? routes.filter(route => filter.test(route))
    : routes

  // remove the /src/routes/ and +page.svelte from the route
  const sanitizedRoutes = filteredRoutes.map(route => {
    return route
      .replace('/src/routes', '')
      .replace('+page.svelte', '')
      .replace('+page.svx', '')
      .replace('+page.md', '')
  })
  return sanitizedRoutes
}

export const routeTail = (route: string) => {
  const segments = route.split('/')
  const segment = segments[segments.length - 2]
  return segment
}

// compare routes ignoring differing trailing slashes
export const compareRoutes = (a: string, b: string) => {
  return a.replace(/\/$/, '') === b.replace(/\/$/, '')
}

export const isActiveRoute = (route: string) => {
  if (browser) {
    return compareRoutes(route, page.url.pathname || '')
  }
}

/**
 * Formats a route into the svelte component name
 * Takes in a lowercase dash separated string and returns pascal case
 */
export const componentNameFromRoute = (route: string) => {
  const tail = routeTail(route)
  const segments = tail.split('-')
  return segments
    .map(
      segment =>
        segment.charAt(0).toUpperCase() + segment.slice(1).toLowerCase(),
    )
    .join('')
}

/**
 * Formats a route into a human readable string
 * Takes in a lowercase dash separated string and returns a capotalized space separated string
 */
export const titleFromRoute = (route: string) => {
  const tail = routeTail(route)
  return tail.replace(/-|_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())
}
