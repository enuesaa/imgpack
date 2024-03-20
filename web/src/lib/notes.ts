import { createQuery } from '@tanstack/svelte-query'

type Note = {
  name: string
}
export const fetchListNotes = () => createQuery<Note[]>({
  queryKey: ['notes'],
  queryFn: async () => {
    return [{ name: 'bb' }]
  },
})
