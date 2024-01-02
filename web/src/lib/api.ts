import { useMutation } from 'react-query'
import type { SignApiRes } from '@/pages/api/sign'
import { useEffect } from 'react'
import type { InvokeApiReq, InvokeApiRes } from '@/pages/api/invoke'

export const useSign = () => useMutation<SignApiRes>({
    mutationFn: async () => {
      const res = await fetch(`http://localhost:3000/api/sign`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      })
      const body = await res.json()
      return body as SignApiRes
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

export const useInvoke = () => useMutation<InvokeApiRes, null, {id: string}>({
    mutationFn: async ({ id }) => {
      const res = await fetch(`http://localhost:3000/api/invoke`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({id} as InvokeApiReq),
      })
      const body = await res.json()
      return body as InvokeApiRes
    },
  })

type UploadResult = {
  id?: string
  url?: string
  converted: boolean
}
export const useUpload = (file: File): UploadResult => {
  const sign = useSign()
  const uploadObject = useUploadObject()
  const invoke = useInvoke()

  useEffect(() => sign.mutate(), [])
  useEffect(() => {
    const url = sign.data?.url ?? ''
    if (url === '') {
      return
    }
    uploadObject.mutate({url, file})
    invoke.mutate({id: sign.data?.id ?? ''})
  }, [sign.data?.url])

  return {
    id: sign.data?.id,
    url: invoke.data?.url,
    converted: invoke.data?.converted ?? false,
  }
}
