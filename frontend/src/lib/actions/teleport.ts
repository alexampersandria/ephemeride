export const teleport = (element: HTMLElement, query: string) => {
  let teleportContainer = document.querySelector(query)
  if (teleportContainer) {
    teleportContainer.appendChild(element)

    return {
      destroy() {
        element.remove()
      },
    }
  }
}
