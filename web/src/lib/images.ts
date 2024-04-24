import { writable } from 'svelte/store'
import { nanoid } from 'nanoid'

type Image = {
  id: string;
  compressedUrl: string;
  filename: string;
}
export const images = writable<Image[]>([])

export const addImage = (image: Omit<Image, 'id'>) => {
  const item = {
    id: nanoid(),
    compressedUrl: image.compressedUrl,
    filename: image.filename,
  }
  images.update(list => [...list, item])
}

export const removeImage = (id: string) => {
  images.update(list => [...list.filter(image => image.id !== id)])
}
