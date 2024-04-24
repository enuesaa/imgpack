import { createMutation } from '@tanstack/svelte-query'

const apiBaseUrl = 'http://localhost:3000/api/'

export const useUpload = () => createMutation({
  mutationFn: async (formdata: FormData): Promise<string> => {
    const res = await fetch(`${apiBaseUrl}upload`, {
      method: 'POST',
      body: formdata
    });
    const compressed = await res.blob()
    return URL.createObjectURL(compressed)
  },
})
