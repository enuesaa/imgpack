import { Box, Container, Text } from '@radix-ui/themes'
import styles from './FileUploadZone.css'
import { ChangeEventHandler, DragEventHandler } from 'react'
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

  const handleChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    e.preventDefault()
    console.log(e)
    if (e.target.files === null || e.target.files.length === 0) {
      return
    }

    for (let i = 0; i < e.target.files.length; i++) {
      const file = e.target.files.item(i)
      if (file === null) {
        continue
      }
      addFile(file)
    }
    e.target.value = ''
  }

  return (
    <Box
      mt='6'
      p='9'
      className={styles.main}
      onDrop={handleDrop}
      onDragOver={handleDragDoNothing}
      onDragEnter={handleDragDoNothing}
      onDragLeave={handleDragDoNothing}
    >
      <input type='file' onChange={handleChange} className={styles.input} />
      <Box>drop here</Box>
    </Box>
  )
}
