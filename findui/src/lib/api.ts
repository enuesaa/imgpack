import { useMutation, useQuery } from 'react-query'

const apiBaseUrl = 'http://localhost:3000/api/'

type ListFilesSchema = {
  path: string
  items: ListFilesSchemaItem[]
}
type ListFilesSchemaItem = {
  name: string
  isCompressable: boolean
  isDir: boolean
}
export const useListFiles = (path: string) => useQuery({
  queryKey: `listFiles-${path}`,
  queryFn: async (): Promise<ListFilesSchema> => {
    const res = await fetch(`${apiBaseUrl}files?path=${path}`)
    const body = await res.json()
    return body
  },
})


type CompressSchema = {
  success: boolean
}
export const useCompress = () => useMutation({
  mutationFn: async (filename: string): Promise<CompressSchema> => {
    const res = await fetch(`${apiBaseUrl}/compress`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        filename,
      }),
    })
    const body = await res.json()
    return body
  },
})
