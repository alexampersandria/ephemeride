export const teleport = (element: HTMLElement, query: string) => {
  const teleportContainer = document.querySelector(query)
  if (teleportContainer) {
    teleportContainer.appendChild(element)

    return {
      destroy() {
        element.remove()
      },
    }
  }
}
