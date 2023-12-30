import { useMutation } from 'react-query'
import type { SignApiResponse } from '@/pages/api/sign'
import { useEffect } from 'react'

export const useSign = () => useMutation<SignApiResponse>({
    mutationFn: async () => {
      const res = await fetch(`http://localhost:3000/api/sign`, {
        method: 'POST',
      })
      const body = await res.json()
      return body as SignApiResponse
    },
  })

export const useUploadObject = () => useMutation({
  mutationFn: async ({url, file}: {url: string, file: File}) => {
    const res = await fetch(url, {
      method: 'PUT',
      headers: {
        'Content-Type': 'image/png',
      },
      body: file,
    })
    const body = await res.json()
    console.log(res)
    console.log(body)
  },
})

type UploadResult = {
  id: string | undefined
}
export const useUpload = (file: File): UploadResult => {
  const sign = useSign()
  const uploadObject = useUploadObject()

  useEffect(() => sign.mutate(), [])
  useEffect(() => {
    const url = sign.data?.url ?? ''
    if (url === '') {
      return
    }
    uploadObject.mutate({url, file})
  }, [sign.data?.url])

  return {id: sign.data?.id}
}
