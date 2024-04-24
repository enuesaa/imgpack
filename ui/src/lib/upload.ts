import { createMutation } from '@tanstack/svelte-query'
import { addImage } from './images'

const apiBaseUrl = 'http://localhost:3000/api/'

export const useUpload = () => createMutation({
  mutationFn: async (files: FileList) => {
    const file = files[0]
    const filename = file.name

		const formdata = new FormData()
		formdata.append('file', file)

    const res = await fetch(`${apiBaseUrl}upload`, {
      method: 'POST',
      body: formdata
    })
    const compressed = await res.blob()

    const compressedUrl = URL.createObjectURL(compressed)
    addImage({ compressedUrl, filename })
  },
})
