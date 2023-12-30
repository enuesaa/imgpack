import { useMutation } from 'react-query'
import type { SignApiResponse } from '@/pages/api/sign'
import { useEffect } from 'react'
import { StatusApiResponse } from '@/pages/api/status'

export const useSign = () => useMutation<SignApiResponse>({
    mutationFn: async () => {
      const res = await fetch(`http://localhost:3000/api/sign`, {
        method: 'POST',
      })
      const body = await res.json()
      return body as SignApiResponse
    },
  })

export const useStatus = () => useMutation<StatusApiResponse, null, {id: string}>({
    mutationFn: async ({ id }) => {
      const res = await fetch(`http://localhost:3000/api/status`, {
        method: 'POST',
        body: JSON.stringify({id}),
      })
      const body = await res.json()
      return body as StatusApiResponse
    },
  })


export const useGetStatus = () => useMutation({
  mutationFn: async ({url}: {url: string}) => {
    const res = await fetch(url, {
      method: 'GET',
    })
    const body = await res.json()
    console.log(res)
    console.log(body)
    return body?.status ?? 'uploading'
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
  status: string
}
export const useUpload = (file: File): UploadResult => {
  const sign = useSign()
  const uploadObject = useUploadObject()
  const status = useStatus()
  const getStatus = useGetStatus()

  useEffect(() => sign.mutate(), [])
  useEffect(() => {
    const url = sign.data?.url ?? ''
    if (url === '') {
      return
    }
    uploadObject.mutate({url, file})
    status.mutate({id: sign.data?.id ?? ''})
  }, [sign.data?.url])
  useEffect(() => getStatus.mutate({url: status.data?.url ?? ''}), [])

  return {id: sign.data?.id, status: getStatus.data ?? 'uploading'}
}
