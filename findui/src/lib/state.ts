import { atom, useAtomValue, useSetAtom } from 'jotai'

const filesAtom = atom<File[]>([])

export const useListFiles = () => useAtomValue(filesAtom)

export const useAddFile = () => {
  const setFiles = useSetAtom(filesAtom)
  const setter = (file: File) => {
    setFiles((files) => [...files, file])
  }

  return setter
}
