import { createQuery, createMutation } from '@tanstack/svelte-query'

const apiBaseUrl = 'http://localhost:3000/api/'

type FilesSchema = {
  path: string
  items: FileItem[]
}
export type FileItem = {
  name: string
  isCompressable: boolean
  isDir: boolean
}
export const listFiles = (path: string) => createQuery<FilesSchema>({
  queryKey: [`listFiles-${path}`],
  queryFn: async () => {
    const res = await fetch(`${apiBaseUrl}files?path=${path}`)
    const body = await res.json()
    return body
  },
})

type CompressSchema = {
  success: boolean
}
export const useCompress = () => createMutation({
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
