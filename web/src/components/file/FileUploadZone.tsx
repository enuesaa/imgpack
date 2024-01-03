import { Box } from '@radix-ui/themes'
import styles from './FileUploadZone.css'
import { DragEventHandler } from 'react'
import { useAddFile } from '@/lib/state'

export const FileUploadZone = () => {
  const addFile = useAddFile()

  const handleDrop: DragEventHandler<HTMLDivElement> = (e) => {
    e.preventDefault()
    e.stopPropagation()
    if (e.dataTransfer.files.length === 0) {
      return
    }

    for (let i = 0; i < e.dataTransfer.files.length; i++) {
      const file = e.dataTransfer.files.item(i)
      if (file === null) {
        continue
      }
      addFile(file)
    }
    e.dataTransfer.clearData()
  }

  const handleDragDoNothing: DragEventHandler<HTMLDivElement> = (e) => {
    e.preventDefault()
    e.stopPropagation()
  }

  return (
    <Box
      mt='4' mb='4' p='5'
      className={styles.main}
      onDrop={handleDrop}
      onDragOver={handleDragDoNothing}
      onDragEnter={handleDragDoNothing}
      onDragLeave={handleDragDoNothing}
    >
      
    </Box>
  )
}
