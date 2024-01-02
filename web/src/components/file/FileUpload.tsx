import { Section } from '@radix-ui/themes'
import { useDropzone } from 'react-dropzone'
import styles from './FileUpload.css'
import { FileUploadCard } from './FileUploadCard'

export const FileUpload = () => {
  const {acceptedFiles, getRootProps, getInputProps} = useDropzone({
    accept: {
      'image/*': ['.png', '.jpeg', '.jpg'],
    },
  })

  return (
    <Section {...getRootProps()} className={styles.main} mt='4' mb='4' p='5'>
      <input {...getInputProps()} />
      {acceptedFiles.map((file, i) => (
        <FileUploadCard key={i} file={file} />
      ))}
    </Section>
  )
}
