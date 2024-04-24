import { createMutation } from '@tanstack/svelte-query'

const apiBaseUrl = 'http://localhost:3000/api/'

export const useUpload = () => createMutation({
  mutationFn: async (formdata: FormData) => {
    const res = await fetch(`${apiBaseUrl}upload`, {
      method: 'POST',
      body: formdata
    });
    console.log(res)
  },
})
