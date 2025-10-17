/**
 * Finds the first focusable element within a given container.
 * @param container an HTML element container
 * @returns the first focusable element inside the container, or null if none found
 */
export const firstFocusable = (container: HTMLElement): HTMLElement | null => {
  const firstElement = container.querySelector<HTMLElement>(
    focusableSelectors.join(', '),
  )
  if (firstElement) {
    return firstElement
  }

  return null
}

/**
 * A list of selectors that match focusable elements.
 */
export const focusableSelectors = [
  '[href]',
  'input:not([disabled])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  'button:not([disabled])',
  'iframe',
  'object',
  'embed',
  '[contenteditable]',
  '[tabindex]:not([tabindex="-1"])',
]
