import { createMutation } from '@tanstack/svelte-query'
import { addImage } from './images'
import { PUBLIC_API_BASE } from '$env/static/public'

export const useUpload = () => createMutation({
  mutationFn: async (files: FileList) => {
    const file = files[0]
    const filename = file.name

		const formdata = new FormData()
		formdata.append('file', file)

    const res = await fetch(`${PUBLIC_API_BASE}upload`, {
      method: 'POST',
      body: formdata
    })
    const compressed = await res.blob()

    const compressedUrl = URL.createObjectURL(compressed)
    addImage({ compressedUrl, filename })
  },
})
