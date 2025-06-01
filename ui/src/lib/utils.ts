type EventHandler = (e: any) => void

// see https://svelte.dev/docs/svelte/v5-migration-guide
export function preventdefault(fn: EventHandler): EventHandler {
  return function (e: Event) {
    e.preventDefault()
    fn(e)
  }
}
